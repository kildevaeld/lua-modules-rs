pub fn write_definitions(path: &std::path::Path) -> std::io::Result<()> {
    lua_json::write_definition(path)?;
    lua_date::write_definition(path)?;
    lua_fs::write_definition(path)?;
    lua_util::write_definition(path)?;
    #[cfg(feature = "shell")]
    lua_shell::write_definition(path)?;
    #[cfg(feature = "crypto")]
    lua_crypto::write_definition(path)?;
    #[cfg(feature = "regex")]
    lua_regexp::write_definition(path)?;
    Ok(())
}

pub fn register_module(vm: &mlua::Lua) -> mlua::Result<()> {
    lua_util::register_modules(vm)?;
    lua_json::register_module(vm)?;
    lua_fs::register_module(vm)?;
    lua_date::register_module(vm)?;
    #[cfg(feature = "http")]
    lua_http::register_module(vm)?;
    #[cfg(feature = "shell")]
    lua_shell::register_module(vm)?;
    #[cfg(feature = "crypto")]
    lua_crypto::register_module(vm)?;
    #[cfg(feature = "regex")]
    lua_regexp::register_module(vm)?;

    Ok(())
}

pub fn create_vm() -> mlua::Result<mlua::Lua> {
    let vm = mlua::Lua::new_with(
        mlua::StdLib::COROUTINE
            | mlua::StdLib::MATH
            | mlua::StdLib::PACKAGE
            | mlua::StdLib::STRING
            | mlua::StdLib::TABLE,
        mlua::LuaOptions::default(),
    )?;

    register_module(&vm)?;

    Ok(vm)
}

pub mod util {
    pub use lua_util::{module, search_path};
}
