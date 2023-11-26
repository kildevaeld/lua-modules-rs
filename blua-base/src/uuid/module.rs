use mlua::UserData;

use super::uuid::LuaUuid;

pub struct Module;

impl UserData for Module {
    fn add_methods<'lua, M: mlua::prelude::LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_function("new", |_, _: ()| {
            let id = uuid::Uuid::new_v4();
            Ok(LuaUuid(id))
        });
    }
}
