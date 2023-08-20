use mlua::{IntoLua, IntoLuaMulti, MetaMethod};

pub struct LuaIter<I>(std::iter::Fuse<I>);

impl<I> LuaIter<I>
where
    I: Iterator,
{
    pub fn new(iter: I) -> LuaIter<I> {
        LuaIter(iter.fuse())
    }
}
impl<I> mlua::UserData for LuaIter<I>
where
    I: Iterator,
    for<'lua> I::Item: mlua::IntoLuaMulti<'lua>,
{
    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_meta_method_mut(MetaMethod::Call, |vm, this, _: ()| {
            //
            match this.0.next() {
                Some(next) => next.into_lua_multi(vm),
                None => Ok(mlua::Value::Nil.into_lua_multi(vm)?),
            }
            // Ok(())
        })
    }
}
