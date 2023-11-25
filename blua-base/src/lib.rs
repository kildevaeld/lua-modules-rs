use std::path::Path;

#[cfg(feature = "crypto")]
pub mod crypto;
pub mod date;
pub mod json;
#[cfg(feature = "regexp")]
pub mod regexp;
#[cfg(feature = "util")]
mod util;

pub fn write_definitions(path: &Path) -> std::io::Result<()> {
    date::write_definition(path)?;
    json::write_definition(path)?;
    #[cfg(feature = "crypto")]
    crypto::write_definition(path)?;
    #[cfg(feature = "regexp")]
    regexp::write_definition(path)?;
    #[cfg(feature = "util")]
    util::write_definition(path)?;

    Ok(())
}

pub fn register(vm: &mlua::Lua) -> mlua::Result<()> {
    date::register_module(vm)?;
    json::register_module(vm)?;
    #[cfg(feature = "crypto")]
    crypto::register_module(vm)?;
    #[cfg(feature = "regexp")]
    regexp::register_module(vm)?;
    #[cfg(feature = "util")]
    util::register_module(vm)?;

    Ok(())
}
