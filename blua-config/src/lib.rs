use mlua::IntoLua;

mod module;

blua_shared::definition!(BLUA_CONFIG("blua.config") = "../definitions/blua.config.lua");

pub fn register_module(vm: &mlua::Lua) -> mlua::Result<()> {
    blua_shared::module::register(vm, "blua.config", |vm| {
        let module = module::init(vm)?;

        module.into_lua(vm)
    })?;

    Ok(())
}
