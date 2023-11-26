mod module;
mod uuid;

use blua_shared::definition;
pub use uuid::LuaUuid;

use mlua::IntoLua;

definition!(BLUA_UUID("blua.uuid") = "../../definitions/blua.uuid.lua");

pub fn register_module(vm: &mlua::Lua) -> mlua::Result<()> {
    blua_shared::module::register(vm, "blua.uuid", |vm| module::Module.into_lua(vm))
}
