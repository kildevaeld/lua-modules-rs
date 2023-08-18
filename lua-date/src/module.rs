use std::time::Duration;

use chrono::{DateTime, Datelike, FixedOffset, Local, NaiveDate, TimeZone, Timelike, Utc};
use mlua::{MetaMethod, ToLua};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum LuaDateTime {
    Utc(DateTime<Utc>),
    Local(DateTime<Local>),
    Fixed(DateTime<FixedOffset>),
}

macro_rules! do_match {
    ($this: expr, |$e:ident| $b:expr) => {
        match $this {
            LuaDateTime::Utc($e) => $b,
            LuaDateTime::Local($e) => $b,
            LuaDateTime::Fixed($e) => $b,
        }
    };
}

macro_rules! do_prop {
    ($fields: expr, $($method: ident)+) => {
        $(
            $fields.add_field_method_get(stringify!($method), |_vm, this| Ok(this.$method()));
        )*
    };
}

impl LuaDateTime {
    fn into_utc(self) -> LuaDateTime {
        let date = match self {
            LuaDateTime::Fixed(fixed) => fixed.naive_utc().and_utc(),
            LuaDateTime::Local(fixed) => fixed.naive_utc().and_utc(),
            LuaDateTime::Utc(fixed) => fixed,
        };

        LuaDateTime::Utc(date)
    }

    pub fn into_datetime(self) -> DateTime<Utc> {
        match self {
            LuaDateTime::Fixed(fixed) => fixed.naive_utc().and_utc(),
            LuaDateTime::Local(fixed) => fixed.naive_utc().and_utc(),
            LuaDateTime::Utc(fixed) => fixed,
        }
    }
}

impl LuaDateTime {
    fn hour(&self) -> u32 {
        do_match!(self, |e| e.hour())
    }

    fn minute(&self) -> u32 {
        do_match!(self, |e| e.minute())
    }

    fn second(&self) -> u32 {
        do_match!(self, |e| e.second())
    }

    fn nanosecond(&self) -> u32 {
        do_match!(self, |e| e.nanosecond())
    }
}

impl LuaDateTime {
    fn year(&self) -> i32 {
        do_match!(self, |e| e.year())
    }

    fn month(&self) -> u32 {
        do_match!(self, |e| e.month())
    }

    fn month0(&self) -> u32 {
        do_match!(self, |e| e.month0())
    }

    fn day(&self) -> u32 {
        do_match!(self, |e| e.day())
    }

    fn day0(&self) -> u32 {
        do_match!(self, |e| e.day0())
    }

    fn ordinal(&self) -> u32 {
        do_match!(self, |e| e.ordinal())
    }

    fn ordinal0(&self) -> u32 {
        do_match!(self, |e| e.ordinal0())
    }

    fn weekday(&self) -> chrono::Weekday {
        do_match!(self, |e| e.weekday())
    }

    fn iso_week(&self) -> chrono::IsoWeek {
        do_match!(self, |e| e.iso_week())
    }
}

impl mlua::UserData for LuaDateTime {
    fn add_fields<'lua, F: mlua::UserDataFields<'lua, Self>>(fields: &mut F) {
        do_prop!(fields, hour minute second nanosecond year month month0 day day0 ordinal ordinal0);
    }

    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("utc", |_, this, _: ()| Ok(this.into_utc()));

        methods.add_meta_method(MetaMethod::ToString, |vm, this, _: ()| {
            Ok(do_match!(this, |e| e.to_rfc3339()))
        });
    }
}

pub struct LuaDate(NaiveDate);

pub struct LuaDuration(Duration);

impl mlua::UserData for LuaDuration {}

pub fn init(vm: &mlua::Lua, module: &mlua::Table<'_>) -> Result<(), mlua::Error> {
    let table = vm.create_table()?;

    let now_local_datetime = vm.create_function(|vm, _: ()| {
        let local: DateTime<Local> = Local::now();
        Ok(LuaDateTime::Local(local))
    })?;

    let parse_from_rfc2822 = vm.create_function(|vm, string: mlua::String| {
        let Ok(date) = DateTime::parse_from_rfc3339(string.to_str()?) else {
            return Ok(mlua::Nil)
        };
        LuaDateTime::Fixed(date).to_lua(vm)
    })?;

    let parse_from_rfc3339 = vm.create_function(|vm, string: mlua::String| {
        let Ok(date) = DateTime::parse_from_rfc3339(string.to_str()?) else {
            return Ok(mlua::Nil)
        };
        LuaDateTime::Fixed(date).to_lua(vm)
    })?;

    table.set("now", now_local_datetime)?;
    table.set("from_rfc2822", parse_from_rfc2822)?;
    table.set("from_rfc3339", parse_from_rfc3339)?;

    module.set("DateTime", table)?;

    Ok(())
}
