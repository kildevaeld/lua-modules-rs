mod file;
use blua_shared::definition;

definition!(CORE_FS("core.fs") = "../../definitions/blua.fs.lua");

pub fn register_module(vm: &mlua::Lua) -> mlua::Result<()> {
    blua_shared::module::register(vm, "blua.fs", |vm| {
        let table = vm.create_table()?;

        file::init(vm, &table)?;

        Ok(mlua::Value::Table(table))
    })?;

    Ok(())
}
