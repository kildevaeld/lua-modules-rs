use mlua::ToLua;

mod module;

lua_util::definition!(CORE_TIME("core.config") = "../definitions/core.config.lua");

pub fn register_module(vm: &mlua::Lua) -> mlua::Result<()> {
    lua_util::module::register(vm, "core.config", |vm| {
        let module = module::init(vm)?;

        module.to_lua(vm)
    })?;

    Ok(())
}
