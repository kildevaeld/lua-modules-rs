use lua_util::definition;
use mlua::IntoLua;

mod exec;
pub mod module;
mod shell;

definition!(CORE_TIME("core.shell") = "../definitions/core.shell.lua");

pub fn register_module(vm: &mlua::Lua) -> mlua::Result<()> {
    lua_util::module::register(vm, "core.shell", |vm| {
        let table = vm.create_table()?;

        let module = module::init(vm, &table)?;

        module.into_lua(vm)
    })?;

    Ok(())
}
