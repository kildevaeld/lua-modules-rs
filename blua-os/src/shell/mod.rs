use blua_shared::definition;
use mlua::IntoLua;
use shell::Shell;

mod exec;
mod shell;

definition!(CORE_TIME("blua.shell") = "../../definitions/blua.shell.lua");

pub fn register_module(vm: &mlua::Lua) -> mlua::Result<()> {
    blua_shared::module::register(vm, "blua.shell", |vm| Shell.into_lua(vm))?;

    Ok(())
}
