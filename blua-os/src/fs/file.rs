use blua_shared::{bytes::LuaBuffer, new_lock, stream::LuaStream, Locket, Lrc};
use futures_lite::{ready, Stream, StreamExt};
use locket::AsyncLockApi;

use std::{os::unix::prelude::FileTypeExt, path::PathBuf, task::Poll};
use tokio::io::{AsyncBufReadExt, AsyncReadExt, BufReader};

pub fn init(vm: &mlua::Lua, module: &mlua::Table<'_>) -> Result<(), mlua::Error> {
    let read_dir = vm.create_async_function(read_dir)?;

    module.set("read_dir", read_dir)?;

    let open_file = vm.create_async_function(open_file)?;

    module.set("open", open_file)?;

    Ok(())
}

pub async fn read_dir(_vm: &mlua::Lua, path: mlua::String<'_>) -> mlua::Result<LuaStream<ReadDir>> {
    let path = path.to_str()?;

    let stream = tokio::fs::read_dir(path).await?;

    let stream = tokio_stream::wrappers::ReadDirStream::new(stream);

    Ok(LuaStream::new(ReadDir { stream }))
}

pub async fn read_file(_vm: &mlua::Lua, path: mlua::String<'_>) -> mlua::Result<LuaBuffer> {
    let path = path.to_str()?;
    let bytes = tokio::fs::read(path).await.map_err(mlua::Error::external)?;
    Ok(bytes.into())
}

pub async fn write_file(
    _vm: &mlua::Lua,
    (path, content): (mlua::String<'_>, mlua::String<'_>),
) -> mlua::Result<()> {
    let path = path.to_str()?;
    tokio::fs::write(path, content)
        .await
        .map_err(mlua::Error::external)?;

    Ok(())
}

async fn open_file(_vm: &mlua::Lua, path: mlua::String<'_>) -> mlua::Result<LuaFile> {
    let path = path.to_str()?;

    let stream = tokio::fs::OpenOptions::default()
        .read(true)
        .open(path)
        .await?;

    Ok(LuaFile {
        file: new_lock(stream),
    })
}

pin_project_lite::pin_project! {
    pub struct ReadDir {
        #[pin]
        stream: tokio_stream::wrappers::ReadDirStream,
    }
}

impl Stream for ReadDir {
    type Item = Result<LuaDirEntry, mlua::Error>;
    fn poll_next(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        let this = self.project();
        match ready!(this.stream.poll_next(cx)) {
            Some(entry) => Poll::Ready(Some(
                entry
                    .map(|m| LuaDirEntry { entry: m.into() })
                    .map_err(mlua::Error::external),
            )),
            None => Poll::Ready(None),
        }
    }
}

#[derive(Clone)]
pub struct LuaDirEntry {
    entry: Lrc<tokio::fs::DirEntry>,
}

impl LuaDirEntry {
    pub fn path(&self) -> PathBuf {
        self.entry.path()
    }
}

impl mlua::UserData for LuaDirEntry {
    fn add_fields<'lua, F: mlua::UserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("path", |_vm, this| {
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
pub struct LuaFile {
    file: Locket<tokio::fs::File>,
}

impl mlua::UserData for LuaFile {
    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_async_method("lines", |_vm, this, _args: ()| async move {
            //
            let file = this.file.read().await.map_err(mlua::Error::external)?;

            let file = file.try_clone().await.map_err(mlua::Error::external)?;

            let buffer = BufReader::new(file);

            let lines = buffer.lines();

            Ok(LuaStream::new(tokio_stream::wrappers::LinesStream::new(
                lines,
            )))
        });

        methods.add_async_method("read", |_vm, this, _args: ()| async move {
            //
            let mut file = this.file.write().await.map_err(mlua::Error::external)?;

            let mut buffer = Vec::default();
            file.read_to_end(&mut buffer).await?;

            Ok(buffer)
        });

        methods.add_async_method("readString", |_vm, this, _args: ()| async move {
            //
            let mut file = this.file.write().await.map_err(mlua::Error::external)?;

            let mut buffer = Default::default();
            file.read_to_string(&mut buffer).await?;

            Ok(buffer)
        });
    }
}
