use blua_shared::definition;
use mlua::IntoLua;

mod module;

definition!(CORE_CRYPTO("blua.crypto") = "../../definitions/blua.crypto.lua");

pub fn register_module(vm: &mlua::Lua) -> mlua::Result<()> {
    blua_shared::module::register(vm, "blua.crypto", |vm| module::Module.into_lua(vm))
}
