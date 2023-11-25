use blua_shared::definition;
use mlua::IntoLua;

pub mod env;
pub mod module;
pub mod settings;

pub use self::{
    env::Environ,
    module::{argv, env, work_dir},
};

definition!(CORE_ENV("core.env") = "../../definitions/blua.env.lua");

pub fn register_module(vm: &mlua::Lua) -> mlua::Result<()> {
    blua_shared::module::register(vm, "blua.env", |vm| module::Module.into_lua(vm))
}
