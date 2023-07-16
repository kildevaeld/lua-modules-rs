use async_trait::async_trait;
use futures_lite::{Stream, StreamExt};
use locket::AsyncLockApi;
use mlua::{MetaMethod, ToLua};

use crate::types::{new_lock, Locket};

pub struct LuaStream<T> {
    file: Locket<T>,
}

impl<T> LuaStream<T> {
    pub fn new(stream: T) -> LuaStream<T> {
        LuaStream {
            file: new_lock(stream),
        }
    }
}

impl<T> Clone for LuaStream<T> {
    fn clone(&self) -> Self {
        LuaStream {
            file: self.file.clone(),
        }
    }
}

impl<T> mlua::UserData for LuaStream<T>
where
    T: DynamicStream + 'static,
    T::Item: Resultable,
    for<'lua> <T::Item as Resultable>::Ok: mlua::ToLua<'lua>,
    <T::Item as Resultable>::Err: std::error::Error + 'static + Send + Sync,
    T: Unpin,
{
    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_async_meta_method(MetaMethod::Call, |vm, this, _: ()| async move {
            let mut lock = this.file.write().await.map_err(mlua::Error::external)?;

            let ret = match lock.next_item().await {
                Some(ret) => ret.into_result().map_err(mlua::Error::external)?,
                None => return Ok(mlua::Value::Nil),
            };

            ret.to_lua(vm)
        });
    }
}

trait Resultable {
    type Ok;
    type Err;

    fn into_result(self) -> Result<Self::Ok, Self::Err>;
}

impl<T, E> Resultable for Result<T, E> {
    type Err = E;
    type Ok = T;

    fn into_result(self) -> Result<T, E> {
        self
    }
}

#[async_trait(?Send)]
trait DynamicStream {
    type Item;
    async fn next_item(&mut self) -> Option<Self::Item>;
}

#[async_trait(?Send)]
impl<T> DynamicStream for T
where
    T: Stream + Unpin,
{
    type Item = T::Item;

    async fn next_item(&mut self) -> Option<Self::Item> {
        <T as StreamExt>::next(self).await
    }
}

#[async_trait(?Send)]
impl<T> DynamicStream for LuaStream<T>
where
    T: Stream + Unpin + 'static,
    T::Item: Resultable,
{
    type Item = T::Item;

    async fn next_item(&mut self) -> Option<Self::Item> {
        let mut lock = self.file.write().await.unwrap();
        lock.next().await
    }
}
