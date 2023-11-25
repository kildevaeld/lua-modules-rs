use std::any::Any;

use crate::{callback::LuaCallback, Return};

pub struct Worker {
    vm: mlua::Lua,
}

impl Worker {
    pub fn new<F>(init: F) -> Result<Worker, mlua::Error>
    where
        F: FnOnce() -> Result<mlua::Lua, mlua::Error>,
    {
        let vm = init()?;

        Ok(Worker { vm })
    }

    pub async fn with_async<F>(&self, func: F) -> Result<(), mlua::Error>
    where
        F: LuaCallback<()>,
    {
        let func = Box::new(func);
        #[cfg(feature = "async")]
        let ret = func.call(&self.vm).await;
        #[cfg(not(feature = "async"))]
        let ret = func.call(&self.vm);
        ret
    }

    pub async fn with_async_ret<F, R>(&self, func: F) -> Result<R, mlua::Error>
    where
        F: LuaCallback<R> + 'static,
        R: 'static,
    {
        let with = Box::new(Return::new(func)) as Box<dyn LuaCallback<Box<dyn Any>>>;

        #[cfg(feature = "async")]
        let ret = with.call(&self.vm).await?;
        #[cfg(not(feature = "async"))]
        let ret = with.call(&self.vm)?;

        if let Ok(ret) = ret.downcast::<R>() {
            Ok(*ret)
        } else {
            Err(mlua::Error::external("could not convert type"))
        }
    }

    pub fn with<F>(&self, func: F) -> Result<(), mlua::Error>
    where
        F: LuaCallback<()>,
    {
        let func = Box::new(func);

        #[cfg(feature = "async")]
        let ret = futures_lite::future::block_on(func.call(&self.vm));
        #[cfg(not(feature = "async"))]
        let ret = func.call(&self.vm);
        ret
    }

    pub fn with_ret<F, R>(&self, func: F) -> Result<R, mlua::Error>
    where
        F: LuaCallback<R> + 'static,
        R: 'static,
    {
        let with = Box::new(Return::new(func)) as Box<dyn LuaCallback<Box<dyn Any>>>;

        #[cfg(feature = "async")]
        let ret = futures_lite::future::block_on(with.call(&self.vm))?;
        #[cfg(not(feature = "async"))]
        let ret = with.call(&self.vm)?;

        if let Ok(ret) = ret.downcast::<R>() {
            Ok(*ret)
        } else {
            Err(mlua::Error::external("could not convert type"))
        }
    }
}
