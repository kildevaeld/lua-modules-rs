pub mod search_path {
    pub fn append(vm: &mlua::Lua, search_path: &str) -> Result<(), mlua::Error> {
        let package = vm.globals().get::<_, mlua::Table>("package")?;

        let path = package.get::<_, String>("path")?;

        let mut split = path.split(';').collect::<Vec<_>>();
        split.push(search_path);

        let path = split.join(";");

        package.set("path", path)?;

        Ok(())
    }

    pub fn list(vm: &mlua::Lua) -> Result<Vec<String>, mlua::Error> {
        let package = vm.globals().get::<_, mlua::Table>("package")?;

        let path = package.get::<_, String>("path")?;

        let split = path.split(';').map(|m| m.to_string()).collect::<Vec<_>>();

        Ok(split)
    }

    pub fn set(vm: &mlua::Lua, search_path: &str) -> Result<(), mlua::Error> {
        let package = vm.globals().get::<_, mlua::Table>("package")?;

        package.set("path", search_path)?;

        Ok(())
    }
}

pub mod module {
    pub fn register<F>(vm: &mlua::Lua, name: &str, func: F) -> Result<(), mlua::Error>
    where
        F: 'static,
        for<'lua> F: Fn(&'lua mlua::Lua) -> Result<mlua::Value<'lua>, mlua::Error>,
    {
        let package = vm
            .globals()
            .get::<_, mlua::Table>("package")?
            .get::<_, mlua::Table>("preload")?;

        let preload = vm.create_function(move |vm, ()| {
            let module = func(vm)?;
            Ok(module)
        })?;

        package.set(name, preload)?;

        Ok(())
    }

    pub fn unregister(vm: &mlua::Lua, name: &str) -> Result<(), mlua::Error> {
        let package = vm
            .globals()
            .get::<_, mlua::Table>("package")?
            .get::<_, mlua::Table>("preload")?;

        package.raw_remove(name)?;

        Ok(())
    }
}
