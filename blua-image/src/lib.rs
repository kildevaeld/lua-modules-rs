mod image;
mod module;

pub use self::image::*;

use blua_shared::definition;
use mlua::IntoLua;

definition!(BLUA_FS("blua.image") = "../definitions/blua.image.lua");

pub fn register_module(vm: &mlua::Lua) -> mlua::Result<()> {
    blua_shared::module::register(vm, "blua.image", |vm| module::Module.into_lua(vm))
}
