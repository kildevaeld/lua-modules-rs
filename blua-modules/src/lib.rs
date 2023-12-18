pub trait ModuleResolver {
    fn resolve(&self);
}

pub trait ModuleLoader {}

use std::sync::Arc;

use mlua::MetaMethod;

struct RequireState {
    resolvers: Vec<Box<dyn ModuleResolver>>,
    loaders: Vec<Box<dyn ModuleLoader>>,
}

pub struct Require {
    state: Arc<RequireState>,
    path: String,
}

impl Require {
    fn resolve(&self, vm: &mlua::Lua, module: &str) -> mlua::Result<String> {
        todo!()
    }

    fn read(&self, vm: &mlua::Lua, module: &str) -> mlua::Result<Vec<u8>> {
        todo!()
    }

    fn create_env<'a>(&self, vm: &'a mlua::Lua) -> mlua::Result<mlua::Value<'a>> {
        let global = vm.create_table()?;

        vm.globals()
            .for_each(|k: mlua::Value, v: mlua::Value| global.set(k, v));

        todo!()
    }
}

impl mlua::UserData for Require {
    fn add_methods<'lua, M: mlua::prelude::LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_meta_method(MetaMethod::Call, |vm, this, module: mlua::String| {
            let resolved = this.resolve(vm, module.to_str()?)?;

            let loaded = this.read(vm, &resolved)?;

            let chunk = vm
                .load(loaded)
                .set_name(resolved)
                .set_environment(this.create_env(vm)?);

            Ok(())
        });
    }
}
