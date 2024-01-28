use std::{
    collections::BTreeMap,
    sync::atomic::{AtomicU16, Ordering},
};

use mlua::RegistryKey;

use crate::{Callback, LuaCallback, LuaExt, WeakWorker};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ItemId(u16);

impl ItemId {
    pub fn new() -> ItemId {
        static COUNTER: AtomicU16 = AtomicU16::new(1);
        ItemId(COUNTER.fetch_add(1, Ordering::SeqCst))
    }
}

#[derive(Clone)]
pub struct Callable {
    worker: WeakWorker,
    id: ItemId,
}

impl Callable {
    pub fn new(vm: &mlua::Lua, func: mlua::Function<'_>) -> mlua::Result<Callable> {
        let call = Callable {
            id: ItemId::new(),
            worker: vm.worker()?,
        };

        let key = vm.create_registry_value(func)?;

        let Some(mut items) = vm.app_data_mut::<BTreeMap<ItemId, RegistryKey>>() else {
            return Err(mlua::Error::external("item registry not registered"));
        };

        items.insert(call.id.clone(), key);

        Ok(call)
    }
}

fn get_func(id: ItemId, vm: &'_ mlua::Lua) -> mlua::Result<mlua::Function<'_>> {
    let Some(items) = vm.app_data_ref::<BTreeMap<ItemId, RegistryKey>>() else {
        return Err(mlua::Error::external("item registry not registered"));
    };

    let Some(item) = items.get(&id) else {
        return Err(mlua::Error::external("function not found in registry"));
    };

    let func: mlua::Function = vm.registry_value(item)?;

    Ok(func)
}

fn remove_func(id: ItemId, vm: &'_ mlua::Lua) -> mlua::Result<()> {
    let Some(mut items) = vm.app_data_mut::<BTreeMap<ItemId, RegistryKey>>() else {
        return Err(mlua::Error::external("item registry not registered"));
    };

    let Some(item) = items.remove(&id) else {
        return Err(mlua::Error::external("function not found in registry"));
    };

    vm.remove_registry_value(item)?;

    Ok(())
}

impl Callable {
    pub fn call<A, R>(&self, args: A) -> mlua::Result<R>
    where
        A: Send + 'static,
        R: Send + 'static,
        for<'lua> A: mlua::IntoLuaMulti<'lua>,
        for<'lua> R: mlua::FromLuaMulti<'lua>,
    {
        let id = self.id;
        #[cfg(not(feature = "async"))]
        let ret = self.worker.with_ret(Callback::new(move |vm| {
            let func = get_func(id, vm)?;
            let ret = func.call::<_, R>(args)?;

            Ok(ret)
        }));

        #[cfg(feature = "async")]
        let ret = self.worker.with_ret(Callback::new(move |vm| {
            Box::pin(async move {
                let func = get_func(id, vm)?;
                let ret = func.call_async::<_, R>(args).await?;

                Ok(ret)
            })
        }));

        ret
    }

    pub async fn call_async<A, R>(&self, args: A) -> mlua::Result<R>
    where
        A: Send + 'static,
        R: Send + 'static,
        for<'lua> A: mlua::IntoLuaMulti<'lua>,
        for<'lua> R: mlua::FromLuaMulti<'lua>,
    {
        let id = self.id;
        #[cfg(not(feature = "async"))]
        let ret = self.worker.with_ret(Callback::new(move |vm| {
            let func = get_func(id, vm)?;
            let ret = func.call::<_, R>(args)?;

            Ok(ret)
        }));

        #[cfg(feature = "async")]
        let ret = self
            .worker
            .with_async_ret(Callback::new(move |vm| {
                Box::pin(async move {
                    let func = get_func(id, vm)?;
                    let ret = func.call_async::<_, R>(args).await?;

                    Ok(ret)
                })
            }))
            .await;

        ret
    }

    pub fn close(&self) -> mlua::Result<()> {
        let id = self.id;
        #[cfg(not(feature = "async"))]
        self.worker
            .with_ret(Callback::new(move |vm| remove_func(id, vm)))?;

        #[cfg(feature = "async")]
        self.worker.with_ret(Callback::new(move |vm| {
            Box::pin(async move { remove_func(id, vm) })
        }))?;

        Ok(())
    }

    pub async fn close_async(&self) -> mlua::Result<()> {
        let id = self.id;
        #[cfg(not(feature = "async"))]
        self.worker
            .with_async_ret(Callback::new(move |vm| remove_func(id, vm)))
            .await?;
        #[cfg(feature = "async")]
        self.worker
            .with_async_ret(Callback::new(move |vm| {
                Box::pin(async move { remove_func(id, vm) })
            }))
            .await?;
        Ok(())
    }
}

impl Drop for Callable {
    fn drop(&mut self) {
        self.close().ok();
    }
}
