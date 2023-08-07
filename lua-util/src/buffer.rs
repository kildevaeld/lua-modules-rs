use bytes::Buf;

pub struct Buffer<B>(pub B);

impl<B> mlua::UserData for Buffer<B>
where
    B: Buf,
{
    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(_methods: &mut M) {}
}
