#[macro_use]
mod macros;

pub mod buffer;
pub mod stream;
pub mod types;
pub mod value;

definition!(
    CORE_STREAM("core.stream") = "../definitions/core.stream.lua"
    CORE_UTIL("core.util") = "../definitions/core.util.lua"
);

const CLASS: &[u8] = include_bytes!("class.lua");
const STREAM: &[u8] = include_bytes!("stream.lua");
const UTIL: &[u8] = include_bytes!("util.lua");

pub fn register_modules(vm: &mlua::Lua) -> Result<(), mlua::Error> {
    module::register(vm, "core.util", |vm| vm.load(UTIL).eval::<mlua::Value>())?;
    module::register(vm, "core.class", |vm| vm.load(CLASS).eval::<mlua::Value>())?;
    module::register(vm, "core.stream", |vm| {
        vm.load(STREAM).eval::<mlua::Value>()
    })?;

    Ok(())
}

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
