use chrono::NaiveTime;
use mlua::UserData;

pub struct LuaTime(NaiveTime);

impl UserData for LuaTime {}
