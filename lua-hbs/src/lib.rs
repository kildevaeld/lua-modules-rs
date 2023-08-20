mod module;

use lua_util::definition;
use mlua::IntoLua;

definition!(CORE_HBS("core.hbs") = "../definitions/core.hbs.lua");

pub fn register_module(vm: &mlua::Lua) -> mlua::Result<()> {
    lua_util::module::register(vm, "core.hbs", |vm| module::Module.into_lua(vm))
}
