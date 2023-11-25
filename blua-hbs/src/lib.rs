mod module;

use blua_shared::definition;
use mlua::IntoLua;

definition!(BLUA_HBS("blua.hbs") = "../definitions/blua.hbs.lua");

pub fn register_module(vm: &mlua::Lua) -> mlua::Result<()> {
    blua_shared::module::register(vm, "blua.hbs", |vm| module::Module.into_lua(vm))
}
