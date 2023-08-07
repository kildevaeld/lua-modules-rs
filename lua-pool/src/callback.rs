#[cfg(feature = "async")]
use core::pin::Pin;
#[cfg(feature = "async")]
use futures_lite::Future;
use std::{any::Any, marker::PhantomData};

#[cfg(feature = "async")]
mod imple {
    use super::*;

    pub trait LuaCallback<T> {
        fn call<'a>(
            self: Box<Self>,
            vm: &'a mlua::Lua,
        ) -> Pin<Box<dyn Future<Output = Result<T, mlua::Error>> + 'a>>;
    }

    pub struct Callback<F, R>
    where
        for<'a> F:
            FnOnce(&'a mlua::Lua) -> Pin<Box<dyn Future<Output = Result<R, mlua::Error>> + 'a>>,
    {
        fun: F,
    }

    impl<F, R> Callback<F, R>
    where
        for<'a> F:
            FnOnce(&'a mlua::Lua) -> Pin<Box<dyn Future<Output = Result<R, mlua::Error>> + 'a>>,
    {
        pub fn new(fun: F) -> Callback<F, R> {
            Callback { fun }
        }
    }

    impl<F, R> LuaCallback<R> for Callback<F, R>
    where
        for<'a> F:
            FnOnce(&'a mlua::Lua) -> Pin<Box<dyn Future<Output = Result<R, mlua::Error>> + 'a>>,
    {
        fn call<'a>(
            self: Box<Self>,
            vm: &'a mlua::Lua,
        ) -> Pin<Box<dyn Future<Output = Result<R, mlua::Error>> + 'a>> {
            (self.fun)(vm)
        }
    }
}

#[cfg(not(feature = "async"))]
mod imple {

    pub trait LuaCallback<T> {
        fn call(self: Box<Self>, vm: &mlua::Lua) -> Result<T, mlua::Error>;
    }

    pub struct Callback<F, R>
    where
        for<'a> F: FnOnce(&'a mlua::Lua) -> Result<R, mlua::Error>,
    {
        fun: F,
    }

    impl<F, R> Callback<F, R>
    where
        for<'a> F: FnOnce(&'a mlua::Lua) -> Result<R, mlua::Error>,
    {
        pub fn new(fun: F) -> Callback<F, R> {
            Callback { fun }
        }
    }

    impl<F, R> LuaCallback<R> for Callback<F, R>
    where
        for<'a> F: FnOnce(&'a mlua::Lua) -> Result<R, mlua::Error>,
    {
        fn call(self: Box<Self>, vm: &mlua::Lua) -> Result<R, mlua::Error> {
            (self.fun)(vm)
        }
    }
}

pub use self::imple::*;

pub(crate) struct Return<F, R> {
    func: Box<F>,
    _p: PhantomData<R>,
}

impl<F, R> Return<F, R> {
    pub fn new(call: F) -> Return<F, R> {
        Return {
            func: Box::new(call),
            _p: PhantomData,
        }
    }
}

#[cfg(feature = "async")]
impl<F, T> LuaCallback<Box<dyn Any + Send>> for Return<F, T>
where
    for<'a> F: LuaCallback<T> + 'a,
    T: 'static + Send,
{
    fn call<'a>(
        self: Box<Self>,
        vm: &'a mlua::Lua,
    ) -> Pin<Box<dyn Future<Output = Result<Box<dyn Any + Send>, mlua::Error>> + 'a>> {
        Box::pin(async move {
            let ret = self.func.call(vm).await?;
            Ok(Box::new(ret) as Box<dyn Any + Send>)
        })
    }
}

#[cfg(feature = "async")]
impl<F, T> LuaCallback<Box<dyn Any>> for Return<F, T>
where
    for<'a> F: LuaCallback<T> + 'a,
    T: 'static,
{
    fn call<'a>(
        self: Box<Self>,
        vm: &'a mlua::Lua,
    ) -> Pin<Box<dyn Future<Output = Result<Box<dyn Any>, mlua::Error>> + 'a>> {
        Box::pin(async move {
            let ret = self.func.call(vm).await?;
            Ok(Box::new(ret) as Box<dyn Any>)
        })
    }
}

#[cfg(not(feature = "async"))]
impl<F, T> LuaCallback<Box<dyn Any + Send>> for Return<F, T>
where
    for<'a> F: LuaCallback<T> + 'a,
    T: 'static + Send,
{
    fn call(self: Box<Self>, vm: &mlua::Lua) -> Result<Box<dyn Any + Send>, mlua::Error> {
        let ret = self.func.call(vm)?;
        Ok(Box::new(ret))
    }
}

#[cfg(not(feature = "async"))]
impl<F, T> LuaCallback<Box<dyn Any>> for Return<F, T>
where
    for<'a> F: LuaCallback<T> + 'a,
    T: 'static,
{
    fn call(self: Box<Self>, vm: &mlua::Lua) -> Result<Box<dyn Any>, mlua::Error> {
        Ok(Box::new(self.func.call(vm)?))
    }
}
