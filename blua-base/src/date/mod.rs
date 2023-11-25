mod date;
mod datetime;
mod duration;
mod init;
mod time;

use blua_shared::definition;

pub use self::{date::LuaDate, datetime::LuaDateTime, time::LuaTime};

definition!(CORE_TIME("blua.time") = "../../definitions/blua.time.lua");

pub fn register_module(vm: &mlua::Lua) -> mlua::Result<()> {
    blua_shared::module::register(vm, "blua.time", |vm| {
        let table = vm.create_table()?;

        init::init(vm, &table)?;

        Ok(mlua::Value::Table(table))
    })?;

    Ok(())
}
