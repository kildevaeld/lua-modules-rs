

use crate::buffer::LuaBuffer;

pub enum Value<'lua> {
    Lua(mlua::Value<'lua>),
    Bytes(LuaBuffer),
}
