use blua_shared::bytes::LuaBuffer;
use image::io::Reader as ImageReader;
use mlua::UserData;
use std::io::Cursor;

use crate::image::LuaImage;
pub struct Module;

impl UserData for Module {
    fn add_methods<'lua, M: mlua::prelude::LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_async_function("open", |_, args: mlua::String| async move {
            let content = tokio::fs::read(args.to_str()?)
                .await
                .map_err(mlua::Error::external)?;

            let img = ImageReader::new(Cursor::new(content))
                .with_guessed_format()
                .map_err(mlua::Error::external)?
                .decode()
                .map_err(mlua::Error::external)?;

            Ok(LuaImage(img))
        });

        methods.add_async_function("new", |_, args: mlua::UserDataRef<LuaBuffer>| async move {
            let img = ImageReader::new(Cursor::new((&*args).as_ref()))
                .with_guessed_format()
                .map_err(mlua::Error::external)?
                .decode()
                .map_err(mlua::Error::external)?;

            Ok(LuaImage(img))
        });
    }
}
