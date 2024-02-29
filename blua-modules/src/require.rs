use crate::{error::LoadError, loader};
use mlua::{MetaMethod, Value};
use std::sync::Arc;

pub struct RequireState {
    pub loaders: Vec<Box<dyn loader::ModuleLoader>>,
}

#[cfg(feature = "async")]
pub struct AsyncRequireState {
    pub loaders: Vec<Box<dyn loader::AsyncModuleLoader>>,
}
pub struct Require<S> {
    state: Arc<S>,
    path: String,
}

impl<S> Require<S> {
    pub fn new(state: S) -> Require<S> {
        Require {
            state: Arc::new(state),
            path: "main".to_string(),
        }
    }
}

impl<S> Require<S>
where
    Self: mlua::UserData,
    S: 'static,
{
    pub fn create_env<'a>(&self, vm: &'a mlua::Lua, module: &str) -> mlua::Result<mlua::Value<'a>> {
        let global = vm.create_table()?;

        for v in vm.globals().pairs::<Value, Value>() {
            let (k, v) = v?;
            global.set(k, v)?;
        }

        global.set(
            "require",
            Require {
                state: self.state.clone(),
                path: module.to_string(),
            },
        )?;

        Ok(Value::Table(global))
    }

    fn get_cache<'a>(&self, vm: &'a mlua::Lua) -> mlua::Result<mlua::Table<'a>> {
        let package: mlua::Table = vm.globals().get("package")?;
        package.get("loaded")
    }

    fn check_cache<'a>(&self, vm: &'a mlua::Lua, path: &str) -> mlua::Result<mlua::Value<'a>> {
        let val: mlua::Value = self.get_cache(vm)?.get(path)?;
        if val.is_nil() {
            return Err(mlua::Error::external("not found"));
        }

        Ok(val)
    }

    fn put_cache<'lua>(
        &self,
        vm: &'lua mlua::Lua,
        name: &str,
        value: mlua::Value<'lua>,
    ) -> mlua::Result<()> {
        let cache = self.get_cache(vm)?;
        cache.set(name, value)?;

        Ok(())
    }
}

impl Require<RequireState> {
    fn resolve(&self, module: &str) -> mlua::Result<String> {
        for loader in &self.state.loaders {
            match loader.resolve(module, Some(&self.path)) {
                Some(ret) => return Ok(ret),
                None => continue,
            }
        }

        Err(mlua::Error::external("module not found"))
    }

    fn read<'lua>(
        &self,
        vm: &'lua mlua::Lua,
        env: Value<'lua>,
        module: &str,
    ) -> mlua::Result<Value<'lua>> {
        for loader in &self.state.loaders {
            match loader.load(vm, env.clone(), module) {
                Ok(ret) => return Ok(ret),
                Err(err) => {
                    if err.is_not_found() {
                        continue;
                    }
                    return Err(mlua::Error::external(err));
                }
            }
        }

        return Err(mlua::Error::external(LoadError::NotFound));
    }
}

#[cfg(feature = "async")]
impl Require<AsyncRequireState> {
    async fn resolve(&self, module: &str) -> mlua::Result<String> {
        for loader in &self.state.loaders {
            match loader.resolve(module, Some(&self.path)).await {
                Some(ret) => return Ok(ret),
                None => continue,
            }
        }

        Err(mlua::Error::external("module not found"))
    }

    async fn read<'lua>(
        &self,
        vm: &'lua mlua::Lua,
        env: Value<'lua>,
        module: &str,
    ) -> mlua::Result<Value<'lua>> {
        for loader in &self.state.loaders {
            match loader.load(vm, env.clone(), module).await {
                Ok(ret) => return Ok(ret),
                Err(err) => {
                    if err.is_not_found() {
                        continue;
                    }
                    return Err(mlua::Error::external(err));
                }
            }
        }

        return Err(mlua::Error::external(LoadError::NotFound));
    }
}

impl mlua::UserData for Require<RequireState> {
    fn add_fields<'lua, F: mlua::prelude::LuaUserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("current", |_, this| Ok(this.path.clone()));
    }
    fn add_methods<'lua, M: mlua::prelude::LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_meta_method(MetaMethod::Call, |vm, this, module: mlua::String| {
            let resolved = this.resolve(module.to_str()?)?;

            if let Ok(cached) = this.check_cache(vm, &resolved) {
                return Ok(cached);
            }

            let env = this.create_env(vm, &resolved)?;

            let value = this.read(vm, env, &resolved)?;

            this.put_cache(vm, &resolved, value.clone())?;

            Ok(value)
        });
    }
}

#[cfg(feature = "async")]
impl mlua::UserData for Require<AsyncRequireState> {
    fn add_fields<'lua, F: mlua::prelude::LuaUserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("current", |_, this| Ok(this.path.clone()));
    }
    fn add_methods<'lua, M: mlua::prelude::LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_async_meta_method(
            MetaMethod::Call,
            |vm, this, module: mlua::String| async move {
                let resolved = this.resolve(module.to_str()?).await?;

                if let Ok(cached) = this.check_cache(vm, &resolved) {
                    return Ok(cached);
                }

                let env = this.create_env(vm, &resolved)?;

                let value = this.read(vm, env, &resolved).await?;

                this.put_cache(vm, &resolved, value.clone())?;

                Ok(value)
            },
        );
    }
}
