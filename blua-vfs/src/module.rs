pub struct Module;

impl mlua::UserData for Module {
    fn add_methods<'lua, M: mlua::prelude::LuaUserDataMethods<'lua, Self>>(methods: &mut M) {}
}
