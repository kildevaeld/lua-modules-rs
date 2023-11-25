use blua_shared::bytes::LuaBuffer;
use ring::digest::{Context, SHA256, SHA512};

pub struct Module;

impl mlua::UserData for Module {
    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_function("sha256", |_vm, args: mlua::String| {
            let mut context = Context::new(&SHA256);
            context.update(args.as_bytes());

            let digest = context.finish();

            let buffer: LuaBuffer = digest.as_ref().into();

            Ok(buffer)
        });

        methods.add_function("sha512", |_vm, args: mlua::String| {
            let mut context = Context::new(&SHA512);
            context.update(args.as_bytes());

            let digest = context.finish();

            let buffer: LuaBuffer = digest.as_ref().into();

            Ok(buffer)
        });

        methods.add_function("md5", |_vm, args: mlua::String| {
            let digest = md5::compute(args.as_bytes());

            let buffer: LuaBuffer = digest.as_ref().into();

            Ok(buffer)
        });
    }
}
