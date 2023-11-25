use chrono::{DateTime, Local, NaiveDate};
use mlua::IntoLua;

use super::datetime::LuaDateTime;

pub fn init(vm: &mlua::Lua, module: &mlua::Table<'_>) -> Result<(), mlua::Error> {
    let new = vm.create_function(|_vm, (year, month, day): (i32, u32, u32)| {
        let local: DateTime<Local> = match NaiveDate::from_ymd_opt(year, month, day) {
            Some(date) => date
                .and_hms_opt(0, 0, 0)
                .unwrap()
                .and_local_timezone(Local)
                .single()
                .expect("local"),
            None => return Err(mlua::Error::external("invalid date")),
        };
        Ok(LuaDateTime::Local(local))
    })?;

    let now_local_datetime = vm.create_function(|_vm, _: ()| {
        let local: DateTime<Local> = Local::now();
        Ok(LuaDateTime::Local(local))
    })?;

    let parse_from_rfc2822 = vm.create_function(|vm, string: mlua::String| {
        let Ok(date) = DateTime::parse_from_rfc3339(string.to_str()?) else {
            return Ok(mlua::Nil);
        };
        LuaDateTime::Fixed(date).into_lua(vm)
    })?;

    let parse_from_rfc3339 = vm.create_function(|vm, string: mlua::String| {
        let Ok(date) = DateTime::parse_from_rfc3339(string.to_str()?) else {
            return Ok(mlua::Nil);
        };
        LuaDateTime::Fixed(date).into_lua(vm)
    })?;

    module.set("new", new)?;
    module.set("now", now_local_datetime)?;
    module.set("from_rfc2822", parse_from_rfc2822)?;
    module.set("from_rfc3339", parse_from_rfc3339)?;

    Ok(())
}
