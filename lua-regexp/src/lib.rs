use regex::Regex;

struct LuaRegex(Regex);

impl mlua::UserData for LuaRegex {}
