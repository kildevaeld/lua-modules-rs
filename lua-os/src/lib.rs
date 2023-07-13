mod module;
mod types;

pub fn register_module(vm: &mlua::Lua) -> mlua::Result<()> {
    let package = vm
        .globals()
        .get::<_, mlua::Table>("package")?
        .get::<_, mlua::Table>("preload")?;

    let preload = vm.create_function(|vm, ()| {
        let table = vm.create_table()?;

        module::init(vm, &table)?;

        Ok(table)
    })?;

    package.set("fs", preload)?;

    Ok(())
}
