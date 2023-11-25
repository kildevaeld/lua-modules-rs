use mlua::MetaMethod;

pub struct LuaMatch {
    pub string: String,
    pub start: usize,
    pub end: usize,
}

impl mlua::UserData for LuaMatch {
    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_meta_method(MetaMethod::ToString, |_, this, args: ()| {
            Ok(this.string.clone())
        })
    }
}
