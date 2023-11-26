use blua_shared::bytes::LuaBuffer;
use mlua::{MetaMethod, UserData};

pub struct LuaUuid(pub uuid::Uuid);

impl UserData for LuaUuid {
    fn add_methods<'lua, M: mlua::prelude::LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_meta_method(MetaMethod::ToString, |_, this, _: ()| {
            Ok(this.0.as_hyphenated().to_string())
        });

        methods.add_method("to_string", |_, this, _: ()| {
            Ok(this.0.as_hyphenated().to_string())
        });

        methods.add_method("to_bytes", |_, this, _: ()| {
            Ok(LuaBuffer(this.0.as_bytes().as_slice().to_vec().into()))
        });
    }
}
