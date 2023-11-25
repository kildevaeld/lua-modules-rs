use chrono::Duration;

pub struct LuaDuration(Duration);

impl mlua::UserData for LuaDuration {}
