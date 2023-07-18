mod client;

pub fn register_module(vm: &mlua::Lua) -> mlua::Result<()> {
    lua_util::module::register(vm, "core.http", |vm| {
        let table = vm.create_table()?;

        client::init(vm, &table)?;

        Ok(mlua::Value::Table(table))
    })?;

    Ok(())
}
