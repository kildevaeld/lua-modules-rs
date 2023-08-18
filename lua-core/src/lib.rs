pub fn write_definitions(path: &std::path::Path) -> std::io::Result<()> {
    lua_json::write_definition(path)?;
    lua_date::write_definition(path)?;
    lua_fs::write_definition(path)?;
    Ok(())
}

pub fn register_module(vm: &mlua::Lua) -> mlua::Result<()> {
    lua_util::register_modules(vm)?;
    lua_json::register_module(vm)?;
    lua_fs::register_module(vm)?;
    lua_date::register_module(vm)?;
    #[cfg(feature = "http")]
    lua_http::register_module(vm)?;
    Ok(())
}

pub mod util {
    pub use lua_util::{module, search_path};
}
