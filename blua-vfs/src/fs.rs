#[cfg(feature = "async")]
use vfs::VAsyncFileExt;
#[cfg(not(feature = "async"))]
use vfs::VFileExt;

#[derive(Debug, Clone)]
pub struct LuaFs<T>(pub T);

#[cfg(feature = "async")]
impl<T> mlua::UserData for LuaFs<T>
where
    T: vfs::VAsyncFS,
    T::Path: 'static,
    <T::Path as vfs::VAsyncPath>::ReadDir: Unpin,
    <T::Path as vfs::VAsyncPath>::File: Unpin,
{
    fn add_methods<'lua, M: mlua::prelude::LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("path", |_, this, path: mlua::String| {
            let path = this.0.path(path.to_str()?).map_err(mlua::Error::external)?;
            Ok(LuaPath(path))
        });
    }
}

#[cfg(not(feature = "async"))]
impl<T> mlua::UserData for LuaFs<T>
where
    T: vfs::VFS + 'static,
{
    fn add_methods<'lua, M: mlua::prelude::LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("path", |_, this, path: mlua::String| {
            let path = this.0.path(path.to_str()?).map_err(mlua::Error::external)?;
            Ok(LuaPath(path))
        });
    }
}

#[derive(Debug, Clone)]
pub struct LuaPath<T>(pub T);

#[cfg(feature = "async")]
impl<T> mlua::UserData for LuaPath<T>
where
    T: vfs::VAsyncPath + 'static,
    T::ReadDir: Unpin,
    T::File: Unpin,
{
    fn add_methods<'lua, M: mlua::prelude::LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("resolve", |_, this, path: mlua::String| {
            let path = this
                .0
                .resolve(path.to_str()?)
                .map_err(mlua::Error::external)?;

            Ok(LuaPath(path))
        });

        methods.add_async_method("metadata", |_, this, _: ()| async {
            let meta = this.0.metadata().await.map_err(mlua::Error::external)?;
            Ok(LuaMetadata(meta))
        });

        methods.add_async_method("exists", |_, this, _: ()| async move {
            Ok(this.0.exists().await)
        });

        methods.add_async_method("read_dir", |_, this, _: ()| async move {
            use blua_shared::stream::DynamicStreamExt;
            let out = this
                .0
                .read_dir()
                .await
                .map_err(mlua::Error::external)?
                .try_map(LuaPath)
                .lua_stream();

            Ok(out)
        });

        methods.add_async_method("open", |_, this, _: ()| async move {
            let out = this
                .0
                .open(vfs::OpenOptions::default().read(true))
                .await
                .map_err(mlua::Error::external)?;

            Ok(LuaFile(out))
        });
    }
}

#[cfg(not(feature = "async"))]
impl<T> mlua::UserData for LuaPath<T>
where
    T: vfs::VPath + 'static,
{
    fn add_methods<'lua, M: mlua::prelude::LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("resolve", |_, this, path: mlua::String| {
            let path = this
                .0
                .resolve(path.to_str()?)
                .map_err(mlua::Error::external)?;

            Ok(LuaPath(path))
        });

        methods.add_method("metadata", |_, this, _: ()| {
            let meta = this.0.metadata().map_err(mlua::Error::external)?;
            Ok(LuaMetadata(meta))
        });

        methods.add_method("exists", |_, this, _: ()| Ok(this.0.exists()));

        methods.add_method("read_dir", |_, this, _: ()| {
            let out = this
                .0
                .read_dir()
                .map_err(mlua::Error::external)?
                .map(|err| err.map(LuaPath))
                .collect::<Result<Vec<_>, _>>()
                .map_err(mlua::Error::external)?;

            Ok(out)
        });

        methods.add_method_mut("open", |_, this, _: ()| {
            let out = this
                .0
                .open(vfs::OpenOptions::default().read(true))
                .map_err(mlua::Error::external)?;

            Ok(LuaFile(out))
        });
    }
}

#[derive(Debug, Clone)]
pub struct LuaFile<T>(pub T);

#[cfg(feature = "async")]
impl<T> mlua::UserData for LuaFile<T>
where
    T: vfs::VAsyncFile + Unpin + 'static,
{
    fn add_methods<'lua, M: mlua::prelude::LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_async_method_mut("read", |_, this, _: ()| async move {
            let mut buffer = Vec::default();
            this.0
                .read_to_end(&mut buffer)
                .await
                .map_err(mlua::Error::external)?;

            Ok(blua_shared::bytes::LuaBuffer(buffer.into()))
        });
    }
}

#[cfg(not(feature = "async"))]
impl<T> mlua::UserData for LuaFile<T>
where
    T: vfs::VFile,
{
    fn add_methods<'lua, M: mlua::prelude::LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method_mut("read", |_, this, _: ()| {
            let mut buffer = Vec::default();
            this.0
                .read_to_end(&mut buffer)
                .map_err(mlua::Error::external)?;

            Ok(blua_shared::bytes::LuaBuffer(buffer.into()))
        });
    }
}

pub struct LuaMetadata(pub vfs::Metadata);

impl mlua::UserData for LuaMetadata {}

#[cfg(feature = "async")]
mod async_impl {
    use super::*;

    #[derive(Debug, Clone)]
    pub struct LuaAsyncFs<T>(pub T);

    impl<T> mlua::UserData for LuaAsyncFs<T>
    where
        T: vfs::VAsyncFS,
        T::Path: 'static,
        <T::Path as vfs::VAsyncPath>::ReadDir: Unpin,
        <T::Path as vfs::VAsyncPath>::File: Unpin,
    {
        fn add_methods<'lua, M: mlua::prelude::LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
            methods.add_method("path", |_, this, path: mlua::String| {
                let path = this.0.path(path.to_str()?).map_err(mlua::Error::external)?;
                Ok(LuaAsyncPath(path))
            });
        }
    }

    #[derive(Debug, Clone)]
    pub struct LuaAsyncPath<T>(pub T);

    impl<T> mlua::UserData for LuaAsyncPath<T>
    where
        T: vfs::VAsyncPath + 'static,
        T::ReadDir: Unpin,
        T::File: Unpin,
    {
        fn add_methods<'lua, M: mlua::prelude::LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
            methods.add_method("resolve", |_, this, path: mlua::String| {
                let path = this
                    .0
                    .resolve(path.to_str()?)
                    .map_err(mlua::Error::external)?;

                Ok(LuaPath(path))
            });

            methods.add_async_method("metadata", |_, this, _: ()| async {
                let meta = this.0.metadata().await.map_err(mlua::Error::external)?;
                Ok(LuaMetadata(meta))
            });

            methods.add_async_method("exists", |_, this, _: ()| async move {
                Ok(this.0.exists().await)
            });

            methods.add_async_method("read_dir", |_, this, _: ()| async move {
                use blua_shared::stream::DynamicStreamExt;
                let out = this
                    .0
                    .read_dir()
                    .await
                    .map_err(mlua::Error::external)?
                    .try_map(LuaPath)
                    .lua_stream();

                Ok(out)
            });

            methods.add_async_method("open", |_, this, _: ()| async move {
                let out = this
                    .0
                    .open(vfs::OpenOptions::default().read(true))
                    .await
                    .map_err(mlua::Error::external)?;

                Ok(LuaFile(out))
            });
        }
    }

    #[derive(Debug, Clone)]
    pub struct LuaAsyncFile<T>(pub T);

    impl<T> mlua::UserData for LuaAsyncFile<T>
    where
        T: vfs::VAsyncFile + Unpin + 'static,
    {
        fn add_methods<'lua, M: mlua::prelude::LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
            methods.add_async_method_mut("read", |_, this, _: ()| async move {
                let mut buffer = Vec::default();
                this.0
                    .read_to_end(&mut buffer)
                    .await
                    .map_err(mlua::Error::external)?;

                Ok(blua_shared::bytes::LuaBuffer(buffer.into()))
            });
        }
    }
}

#[cfg(feature = "async")]
pub use self::async_impl::*;
