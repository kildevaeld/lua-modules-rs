use mlua::IntoLua;

mod module;

lua_util::definition!(CORE_TIME("core.config") = "../definitions/core.config.lua");

pub fn register_module(vm: &mlua::Lua) -> mlua::Result<()> {
    lua_util::module::register(vm, "core.config", |vm| {
        let module = module::init(vm)?;

        module.into_lua(vm)
    })?;

    Ok(())
}
