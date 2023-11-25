use lua_util::definition;
use mlua::IntoLua;
use shell::Shell;

mod exec;
mod shell;

definition!(CORE_TIME("core.shell") = "../definitions/core.shell.lua");

pub fn register_module(vm: &mlua::Lua) -> mlua::Result<()> {
    lua_util::module::register(vm, "core.shell", |vm| Shell.into_lua(vm))?;

    Ok(())
}
