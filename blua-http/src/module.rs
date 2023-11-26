use mlua::UserData;

use crate::response::LuaResponse;

pub struct Module;

macro_rules! method {
    ($builder: expr, $($method: ident)*) => {
        $(
            $builder.add_async_function(stringify!($method), |_, url: mlua::String| async move {
                let resp = reqwest::get(url.to_str()?)
                    .await
                    .map_err(mlua::Error::external)?;

                Ok(LuaResponse(resp))
            });
        )*
    };
}

impl UserData for Module {
    fn add_methods<'lua, M: mlua::prelude::LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        method!(methods, get post put delete head option);
    }
}
