use futures_lite::StreamExt;
use locket::{AsyncLockApi, AsyncLocket};
use mlua::MetaMethod;
use std::{ffi::OsStr, os::unix::prelude::FileTypeExt, path::PathBuf, str::FromStr, sync::Arc};

use crate::types::{new_lock, Locket, Lrc};

pub fn init(vm: &mlua::Lua, module: &mlua::Table<'_>) -> Result<(), mlua::Error> {
    let read_dir = vm.create_async_function(read_dir)?;

    module.set("read_dir", read_dir)?;

    Ok(())
}

async fn read_dir(vm: &mlua::Lua, path: mlua::String<'_>) -> mlua::Result<ReadDir> {
    let path = path.to_str()?;

    let stream = tokio::fs::read_dir(path).await?;

    Ok(ReadDir {
        stream: new_lock(stream),
    })
}

async fn read_file(vm: &mlua::Lua, path: mlua::String<'_>) -> mlua::Result<ReadDir> {
    let path = path.to_str()?;

    let stream = tokio::fs::read_dir(path).await?;

    Ok(ReadDir {
        stream: new_lock(stream),
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
