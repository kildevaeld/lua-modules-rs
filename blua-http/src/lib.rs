mod module;
mod response;
mod types;
mod util;

use blua_shared::definition;
use mlua::IntoLua;

definition!(BLUA_HTTP("blua.http") = "../definitions/blua.http.lua");

pub fn register_module(vm: &mlua::Lua) -> mlua::Result<()> {
    blua_shared::module::register(vm, "blua.http", |vm| module::Module.into_lua(vm))
}
