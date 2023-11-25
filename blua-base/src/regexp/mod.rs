use blua_shared::definition;
use mlua::IntoLua;

mod captures;
mod init;
mod r#match;
mod regex;

pub use self::{captures::LuaCaptures, r#match::LuaMatch, regex::LuaRegex};

definition!(CORE_JSON("blua.regexp") = "../../definitions/blua.regex.lua");

pub fn register_module(vm: &mlua::Lua) -> mlua::Result<()> {
    blua_shared::module::register(vm, "blua.regexp", |vm| {
        let table = vm.create_table()?;

        init::init(vm, &table)?;

        table.into_lua(vm)
    })
}
