use std::{collections::HashMap, env::Args, future::Future, pin::Pin};

use gerning::{
    service::{HasState, State},
    signature::Signature,
    Value,
};
use lua_worker::Callable;
pub struct LuaCallable<V: Value> {
    func: Callable,
    signature: Signature<V>,
}

impl<C, V> gerning::Callable<C, V> for LuaCallable<V>
where
    V: Value + Send + 'static + Clone,
    V::Type: Clone,
    C: Clone + Send + 'static,
    for<'lua> V: mlua::IntoLua<'lua> + mlua::FromLua<'lua>,
    for<'lua> C: mlua::IntoLua<'lua>,
{
    fn signature(&self) -> gerning::signature::Signature<V> {
        self.signature.clone()
    }

    fn call(
        &self,
        ctx: &mut C,
        args: gerning::arguments::Arguments<V>,
    ) -> Result<V, gerning::Error<V>> {
        let ret = self
            .func
            .call::<_, V>((ctx.clone(), LuaArgs { args }))
            .expect("call");

        Ok(ret)
    }
}

#[cfg(feature = "async")]
impl<C, V> gerning::AsyncCallable<C, V> for LuaCallable<V>
where
    V: Value + Send + 'static + Clone,
    V::Type: Clone,
    C: Clone + Send + 'static,
    for<'lua> V: mlua::IntoLua<'lua> + mlua::FromLua<'lua>,
    for<'lua> C: mlua::IntoLua<'lua>,
{
    type Future<'a> = Pin<Box<dyn Future<Output = Result<V, gerning::Error<V>>> + 'a>>;

    fn signature(&self) -> Signature<V> {
        self.signature.clone()
    }

    fn call_async<'a>(
        &'a self,
        ctx: &'a mut C,
        args: gerning::arguments::Arguments<V>,
    ) -> Self::Future<'a> {
        Box::pin(async move {
            let ret = self
                .func
                .call_async::<_, V>((ctx.clone(), LuaArgs { args }))
                .await
                .expect("call");

            Ok(ret)
        })
    }
}

pub struct LuaMethod<V: Value> {
    func: Callable,
    signature: Signature<V>,
}

impl<S, C, V> gerning::service::MethodCallable<S, C, V> for LuaMethod<V>
where
    V: Value + Send + 'static + Clone,
    V::Type: Clone,
    C: Clone + Send + 'static,
    for<'lua> V: mlua::IntoLua<'lua> + mlua::FromLua<'lua>,
    for<'lua> C: mlua::IntoLua<'lua>,
{
    fn signature(&self) -> gerning::signature::Signature<V> {
        self.signature.clone()
    }

    fn call(
        &self,
        this: &mut S,
        ctx: &mut C,
        args: gerning::arguments::Arguments<V>,
    ) -> Result<V, gerning::Error<V>> {
        let ret = self
            .func
            .call::<_, V>((ctx.clone(), LuaArgs { args }))
            .expect("call");

        Ok(ret)
    }
}

// #[cfg(feature = "async")]
// impl<S, C, V> gerning::service::AsyncMethodCallable<S, C, V> for LuaMethod<V>
// where
//     V: Value + Send + 'static + Clone,
//     V::Type: Clone,
//     C: Clone + Send + 'static,
//     for<'lua> V: mlua::IntoLua<'lua> + mlua::FromLua<'lua>,
//     for<'lua> C: mlua::IntoLua<'lua>,
// {
//     type Future<'a> = Pin<Box<dyn Future<Output = Result<V, gerning::Error<V>>> + 'a>>;

//     fn signature(&self) -> Signature<V> {
//         self.signature.clone()
//     }

//     fn call_async<'a>(
//         &'a self,
//         this: &mut S,
//         ctx: &'a mut C,
//         args: gerning::arguments::Arguments<V>,
//     ) -> Self::Future<'a> {
//         Box::pin(async move {
//             let ret = self
//                 .func
//                 .call_async::<_, V>((ctx.clone(), LuaArgs { args }))
//                 .await
//                 .expect("call");

//             Ok(ret)
//         })
//     }
// }

struct LuaArgs<V> {
    args: gerning::arguments::Arguments<V>,
}

impl<'lua, V> mlua::IntoLuaMulti<'lua> for LuaArgs<V>
where
    V: mlua::IntoLua<'lua>,
{
    fn into_lua_multi(
        self,
        lua: &'lua mlua::Lua,
    ) -> mlua::prelude::LuaResult<mlua::MultiValue<'lua>> {
        let mut multi = mlua::MultiValue::new();

        for value in self.args.into_iter() {
            multi.push_front(value.into_lua(lua)?);
        }

        Ok(multi)
    }
}

pub fn create_callable<'lua, V: Value + mlua::FromLua<'lua>>(
    vm: &'lua mlua::Lua,
    args: mlua::Table<'lua>,
) -> mlua::Result<LuaCallable<V>>
where
    V::Type: mlua::FromLua<'lua>,
{
    let params: Option<Vec<V::Type>> = args.get("params")?;
    let returns: V::Type = args.get("returns")?;
    let func: mlua::Function = args.get("call")?;
    let call = lua_worker::Callable::new(vm, func)?;

    let mut builder = gerning::signature::Parameters::<V>::build();

    let params = params.unwrap_or_default();

    for p in params {
        builder.add(p);
    }

    let sig = gerning::signature::Signature::new(builder.build(), returns);
    Ok(LuaCallable {
        func: call,
        signature: sig,
    })
}

pub struct ServiceBuilder<S: gerning::service::ServiceType, T: HasState, C, V: Value> {
    builder: gerning::service::DynService<T, S, C, V>,
}

impl<T, C, V> mlua::UserData for ServiceBuilder<gerning::service::Sync, T, C, V>
where
    V: Value,
    T: HasState,
    T::State: State<V>,
{
    fn add_methods<'lua, M: mlua::prelude::LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method_mut(
            "register",
            |vm, this, (name, args): (mlua::String, mlua::Table)| {
                //
                this.builder
                    .register(name.to_str()?, create_callable(vm, args)?);
                Ok(())
            },
        );
    }
}
