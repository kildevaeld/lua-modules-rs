pub fn init<'a>(vm: &'a mlua::Lua) -> mlua::Result<mlua::Value<'a>> {
    let mut module = vm.create_table()?;

    let ls = vm.create_function(|ctx, args: (mlua::String,)| {
        //
        Ok(())
    })?;

    module.set("ls", ls)?;

    let exec = vm.create_function(|ctx, args: ()| {
        //
        Ok(())
    })?;

    module.set("exec", exec)?;

    Ok(mlua::Value::Table(module))
}

pub fn register_module(vm: &mlua::Lua) -> mlua::Result<()> {
    let package = vm
        .globals()
        .get::<_, mlua::Table>("package")?
        .get::<_, mlua::Table>("preload")?;

    let preload = vm.create_function(|vm, ()| {
        let module = init(vm);

        Ok(module)
    })?;

    package.set("shell", preload)?;

    Ok(())
}
