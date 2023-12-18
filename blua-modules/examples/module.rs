use std::{path::PathBuf, str::FromStr};

use blua_modules::Modules;
use mlua::{LuaOptions, StdLib};

fn main() -> mlua::Result<()> {
    let vm = mlua::Lua::new_with(StdLib::STRING | StdLib::TABLE, LuaOptions::default())?;

    let mut modules = Modules::default();

    modules.register(blua_modules::DirLoader::new(
        PathBuf::from_str("blua-modules/examples").unwrap(),
    ));

    modules.build(&vm)?;

    vm.load(include_str!("test.lua")).eval::<()>()?;

    Ok(())
}
