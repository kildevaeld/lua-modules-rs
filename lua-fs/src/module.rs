use crate::types::{new_lock, Locket, Lrc};
use async_trait::async_trait;
use futures_lite::{Stream, StreamExt};
use locket::{AsyncLockApi, AsyncLocket};
use mlua::{Lua, MetaMethod, RegistryKey, ToLua};
use std::{ffi::OsStr, os::unix::prelude::FileTypeExt, path::PathBuf, str::FromStr, sync::Arc};
use tokio::io::{AsyncBufReadExt, AsyncReadExt, BufReader, ReadBuf};

pub fn init(vm: &mlua::Lua, module: &mlua::Table<'_>) -> Result<(), mlua::Error> {
    let read_dir = vm.create_async_function(read_dir)?;

    module.set("read_dir", read_dir)?;

    let open_file = vm.create_async_function(open_file)?;

    module.set("open", open_file)?;

    Ok(())
}

async fn read_dir(vm: &mlua::Lua, path: mlua::String<'_>) -> mlua::Result<ReadDir> {
    let path = path.to_str()?;

    let stream = tokio::fs::read_dir(path).await?;

    Ok(ReadDir {
        stream: new_lock(stream),
    })
}

async fn open_file(vm: &mlua::Lua, path: mlua::String<'_>) -> mlua::Result<File> {
    let path = path.to_str()?;

    let stream = tokio::fs::OpenOptions::default()
        .read(true)
        .open(path)
        .await?;

    Ok(File {
        file: new_lock(stream),
    })
}

#[derive(Clone)]
struct ReadDir {
    stream: Locket<tokio::fs::ReadDir>,
}

impl mlua::UserData for ReadDir {
    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_async_meta_method(MetaMethod::Call, |vm, this, _args: ()| async move {
            let mut lock = this.stream.write().await.map_err(mlua::Error::external)?;
            let next = lock.next_entry().await?;
            Ok(next.map(|m| DirEntry { entry: m.into() }))
        });
    }
}

#[derive(Clone)]
struct DirEntry {
    entry: Lrc<tokio::fs::DirEntry>,
}

impl mlua::UserData for DirEntry {
    fn add_fields<'lua, F: mlua::UserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("path", |vm, this| {
            let path = this.entry.path();
            Ok(path.to_string_lossy().to_string())
        });
    }

    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_async_method("type", |_vm, this, _: ()| async move {
            //
            let ty = this.entry.file_type().await?;

            let ty = if ty.is_dir() {
                "dir"
            } else if ty.is_file() {
                "file"
            } else if ty.is_symlink() {
                "symlink"
            } else if ty.is_block_device() {
                "block-device"
            } else if ty.is_char_device() {
                "char-device"
            } else if ty.is_fifo() {
                "fifo"
            } else if ty.is_socket() {
                "socket"
            } else {
                "unknown"
            };

            Ok(ty)
        });
    }
}

#[derive(Clone)]
pub struct File {
    file: Locket<tokio::fs::File>,
}

impl mlua::UserData for File {
    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_async_method("lines", |vm, mut this, args: ()| async move {
            //

            let file = this.file.read().await.unwrap().try_clone().await.unwrap();

            let buffer = BufReader::new(file);

            let lines = buffer.lines();

            Ok(LuaStream {
                file: new_lock(tokio_stream::wrappers::LinesStream::new(lines)),
            })
        });
    }
}

pub struct LuaStream<T> {
    file: Locket<T>,
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
    T: Stream + 'static,
    T::Item: Resultable,
    for<'lua> <T::Item as Resultable>::Ok: mlua::ToLua<'lua>,
    <T::Item as Resultable>::Err: std::error::Error + 'static + Send + Sync,
    T: Unpin,
{
    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_async_meta_method(MetaMethod::Call, |vm, this, _: ()| async move {
            let mut lock = this.file.write().await.map_err(mlua::Error::external)?;

            let ret = match lock.next().await {
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
