use blua_shared::{definition, module};

definition!(
    BLUA_STREAM("blua.stream") = "../../definitions/blua.stream.lua"
    BLUA_UTIL("blua.util") = "../../definitions/blua.util.lua"
);

const CLASS: &[u8] = include_bytes!("class.lua");
const STREAM: &[u8] = include_bytes!("stream.lua");
const UTIL: &[u8] = include_bytes!("util.lua");

pub fn register_module(vm: &mlua::Lua) -> Result<(), mlua::Error> {
    module::register(vm, "blua.util", |vm| vm.load(UTIL).eval::<mlua::Value>())?;
    module::register(vm, "blua.class", |vm| vm.load(CLASS).eval::<mlua::Value>())?;
    module::register(vm, "blua.stream", |vm| {
        vm.load(STREAM).eval::<mlua::Value>()
    })?;

    Ok(())
}
