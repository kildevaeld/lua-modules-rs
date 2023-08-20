use lua_util::definition;
use mlua::ToLua;

pub mod module;

definition!(CORE_CRYPTO("core.crypto") = "../definitions/core.crypto.lua");

pub fn register_module(vm: &mlua::Lua) -> mlua::Result<()> {
    lua_util::module::register(vm, "core.crypto", |vm| {
        //
        module::Module.to_lua(vm)
    })
}
