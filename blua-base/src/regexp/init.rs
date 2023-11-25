use blua_shared::definition;
use mlua::IntoLua;

use super::regex::LuaRegex;

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
