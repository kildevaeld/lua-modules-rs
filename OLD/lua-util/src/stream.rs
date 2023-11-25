use async_trait::async_trait;
use futures_lite::{Stream, StreamExt};
use locket::AsyncLockApi;
use mlua::{IntoLua, MetaMethod};

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
    for<'lua> <T::Item as Resultable>::Ok: mlua::IntoLua<'lua>,
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

            ret.into_lua(vm)
        });
    }
}

pub trait Resultable {
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
pub trait DynamicStream {
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

pub trait DynamicStreamExt: DynamicStream {
    fn map<F>(self, map: F) -> MapStream<Self, F>
    where
        Self: Sized,
    {
        MapStream { stream: self, map }
    }

    fn try_map<F>(self, map: F) -> TryMapStream<Self, F>
    where
        Self: Sized,
    {
        TryMapStream { stream: self, map }
    }

    fn lua_stream(self) -> LuaStream<Self>
    where
        Self: Sized,
    {
        LuaStream::new(self)
    }
}

impl<T> DynamicStreamExt for T where T: DynamicStream {}

pub struct MapStream<T, F> {
    stream: T,
    map: F,
}

#[async_trait(?Send)]
impl<T, F, R> DynamicStream for MapStream<T, F>
where
    F: Fn(T::Item) -> R,
    T: DynamicStream,
{
    type Item = R;

    async fn next_item(&mut self) -> Option<Self::Item> {
        let item = self.stream.next_item().await?;
        Some((self.map)(item))
    }
}

pub struct TryMapStream<T, F> {
    stream: T,
    map: F,
}

#[async_trait(?Send)]
impl<T, F, R> DynamicStream for TryMapStream<T, F>
where
    F: Fn(<T::Item as Resultable>::Ok) -> R,
    T: DynamicStream,
    T::Item: Resultable,
{
    type Item = Result<R, <T::Item as Resultable>::Err>;

    async fn next_item(&mut self) -> Option<Self::Item> {
        let item = match self.stream.next_item().await?.into_result() {
            Ok(ret) => ret,
            Err(err) => return Some(Err(err)),
        };
        Some(Ok((self.map)(item)))
    }
}
