mod regexp;

use lua_util::definition;
use mlua::IntoLua;
pub use regexp::LuaRegex;

definition!(CORE_JSON("core.regexp") = "../definitions/blur.regex.lua");

pub fn init(vm: &mlua::Lua, module: &mlua::Table<'_>) -> Result<(), mlua::Error> {
    let func = vm.create_function(|_vm, args: String| {
        let Ok(reg) = regex::Regex::new(&args) else {
            return Ok(None);
        };

        Ok(Some(LuaRegex(reg)))
    })?;

    module.set("new", func)?;

    Ok(())
}

pub fn register_module(vm: &mlua::Lua) -> mlua::Result<()> {
    lua_util::module::register(vm, "core.regexp", |vm| {
        let table = vm.create_table()?;

        init(vm, &table)?;

        table.into_lua(vm)
    })
}
