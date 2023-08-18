use lua_util::definition;

pub mod module;

definition!(CORE_TIME("core.time") = "../definitions/core.time.lua");

pub fn register_module(vm: &mlua::Lua) -> mlua::Result<()> {
    lua_util::module::register(vm, "core.time", |vm| {
        let table = vm.create_table()?;

        module::init(vm, &table)?;

        Ok(mlua::Value::Table(table))
    })?;

    Ok(())
}
