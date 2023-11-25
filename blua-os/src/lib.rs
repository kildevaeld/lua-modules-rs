use std::path::Path;

pub mod env;
pub mod fs;
pub mod shell;

pub fn write_definitions(path: &Path) -> std::io::Result<()> {
    env::write_definition(path)?;
    fs::write_definition(path)?;
    shell::write_definition(path)?;

    Ok(())
}

pub fn register(vm: &mlua::Lua) -> mlua::Result<()> {
    env::register_module(vm)?;
    fs::register_module(vm)?;
    shell::register_module(vm)?;
    Ok(())
}
