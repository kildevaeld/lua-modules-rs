use lua_util::definition;
use mlua::IntoLua;

pub mod env;
pub mod module;
pub mod settings;

definition!(CORE_ENV("core.env") = "../definitions/core.env.lua");

pub fn register_module(vm: &mlua::Lua) -> mlua::Result<()> {
    lua_util::module::register(vm, "core.env", |vm| module::Module.into_lua(vm))
}
