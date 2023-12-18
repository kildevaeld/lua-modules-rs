#[cfg(feature = "async")]
use crate::{require::AsyncRequireState, AsyncModuleLoader};
use crate::{
    require::{Require, RequireState},
    ModuleLoader,
};

#[derive(Default)]
pub struct Modules {
    loaders: Vec<Box<dyn ModuleLoader>>,
}

impl Modules {
    pub fn register<M>(&mut self, loader: M) -> &mut Self
    where
        M: ModuleLoader + 'static,
    {
        self.loaders.push(Box::new(loader));
        self
    }

    pub fn build(self, vm: &mlua::Lua) -> mlua::Result<()> {
        vm.globals().set(
            "require",
            Require::new(RequireState {
                loaders: self.loaders,
            }),
        )
    }
}

#[cfg(feature = "async")]
#[derive(Default)]
pub struct AsyncModules {
    loaders: Vec<Box<dyn AsyncModuleLoader>>,
}

#[cfg(feature = "async")]
impl AsyncModules {
    pub fn register<M>(&mut self, loader: M) -> &mut Self
    where
        M: AsyncModuleLoader + 'static,
    {
        self.loaders.push(Box::new(loader));
        self
    }

    pub fn build(self, vm: &mlua::Lua) -> mlua::Result<()> {
        vm.globals().set(
            "require",
            Require::new(AsyncRequireState {
                loaders: self.loaders,
            }),
        )
    }
}
