mod convert;

pub use convert::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Val(value::Value);

impl Val {
    pub fn new<I>(value: impl Into<value::Value>) -> Val {
        Val(value.into())
    }
}

impl std::ops::Deref for Val {
    type Target = value::Value;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for Val {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<Val> for value::Value {
    fn from(v: Val) -> Self {
        v.0
    }
}

impl From<value::Value> for Val {
    fn from(v: value::Value) -> Self {
        Val(v)
    }
}

impl<'lua> mlua::FromLua<'lua> for Val {
    fn from_lua(lua_value: mlua::Value<'lua>, _lua: &'lua mlua::Lua) -> mlua::Result<Self> {
        Ok(convert::from_lua(lua_value)?.into())
    }
}

impl<'lua> mlua::ToLua<'lua> for Val {
    fn to_lua(self, lua: &'lua mlua::Lua) -> mlua::Result<mlua::Value<'lua>> {
        Ok(convert::to_lua(lua, self.0))
    }
}
