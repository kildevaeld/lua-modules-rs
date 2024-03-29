use blua_shared::definition;
use mlua::{IntoLua, LuaSerdeExt};

definition!(CORE_JSON("blua.json") = "../../definitions/blua.json.lua");

pub fn init(vm: &mlua::Lua, module: &mlua::Table<'_>) -> Result<(), mlua::Error> {
    let parse = vm.create_function(|vm, args: mlua::String| {
        let value: serde_json::Value =
            serde_json::from_slice(args.as_bytes()).map_err(mlua::Error::external)?;

        vm.to_value(&value)
    })?;

    let write = vm.create_function(|vm, (v, pretty): (mlua::Value, Option<bool>)| {
        let value: serde_json::Value = vm.from_value(v)?;

        let json = if pretty == Some(true) {
            serde_json::to_string_pretty(&value)
        } else {
            serde_json::to_string(&value)
        }
        .map_err(mlua::Error::external)?;

        Ok(json)
    })?;

    module.set("decode", parse)?;
    module.set("encode", write)?;

    Ok(())
}

pub fn register_module(vm: &mlua::Lua) -> mlua::Result<()> {
    blua_shared::module::register(vm, "blua.json", |vm| {
        let table = vm.create_table()?;

        init(vm, &table)?;

        table.into_lua(vm)
    })
}
