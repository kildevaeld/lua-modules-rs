pub mod module;

pub fn register_module(vm: &mlua::Lua) -> mlua::Result<()> {
    lua_util::module::register(vm, "core.time", |vm| {
        let table = vm.create_table()?;

        module::init(vm, &table)?;

        Ok(mlua::Value::Table(table))
    })?;

    Ok(())
}
