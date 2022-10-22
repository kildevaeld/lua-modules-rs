use super::convert;

#[derive(Debug, Clone, PartialEq)]
pub struct Val(value::Value);

impl Val {
    pub fn new(value: impl Into<value::Value>) -> Val {
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

#[cfg(feature = "types")]
impl value_types::FromValue for Val {
    type Error = std::convert::Infallible;

    fn from_value(value: value::Value) -> Result<Self, Self::Error> {
        Ok(Val(value))
    }
}

#[cfg(feature = "types")]
impl value_types::IntoValue for Val {
    type Error = std::convert::Infallible;

    fn into_value(self) -> Result<value::Value, Self::Error> {
        Ok(self.0)
    }
}

impl<'lua> mlua::FromLua<'lua> for Val {
    fn from_lua(lua_value: mlua::Value<'lua>, _lua: &'lua mlua::Lua) -> mlua::Result<Self> {
        Ok(convert::from_lua(lua_value)?.into())
    }
}

impl<'lua> mlua::ToLua<'lua> for Val {
    fn to_lua(self, lua: &'lua mlua::Lua) -> mlua::Result<mlua::Value<'lua>> {
        Ok(convert::to_lua(lua, self.0)?)
    }
}
