pub fn register_module(vm: &mlua::Lua) -> mlua::Result<()> {
    lua_util::register_modules(vm)?;
    lua_json::register_module(vm)?;
    lua_fs::register_module(vm)?;
    #[cfg(feature = "http")]
    lua_http::register_module(vm)?;
    Ok(())
}

pub mod util {
    pub use lua_util::{module, search_path};
}
