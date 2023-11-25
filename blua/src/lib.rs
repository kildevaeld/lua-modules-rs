pub fn write_definitions(path: &std::path::Path) -> std::io::Result<()> {
    blua_base::write_definitions(path)?;
    #[cfg(feature = "os")]
    blua_os::write_definitions(path)?;
    Ok(())
}

pub fn register_module(vm: &mlua::Lua) -> mlua::Result<()> {
    blua_base::register(vm)?;
    #[cfg(feature = "os")]
    blua_os::register(vm)?;
    Ok(())
}

pub fn create_vm(options: mlua::LuaOptions) -> mlua::Result<mlua::Lua> {
    let libs =
        mlua::StdLib::MATH | mlua::StdLib::PACKAGE | mlua::StdLib::STRING | mlua::StdLib::TABLE;

    #[cfg(any(
        feature = "lua54",
        feature = "lua53",
        feature = "lua52",
        feature = "luau"
    ))]
    let libs = libs | mlua::StdLib::COROUTINE;

    #[cfg(feature = "luajit")]
    let libs = libs | mlua::StdLib::JIT;

    #[cfg(any(feature = "lua54", feature = "lua53", feature = "luau"))]
    let libs = libs | mlua::StdLib::UTF8;

    let vm = mlua::Lua::new_with(libs, options)?;

    register_module(&vm)?;

    Ok(vm)
}

pub mod util {
    pub use blua_shared::{module, search_path};
}

pub use blua_base as base;
#[cfg(feature = "os")]
pub use blua_os as os;
