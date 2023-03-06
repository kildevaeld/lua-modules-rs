use deadpool::{async_trait, managed};
use std::sync::Arc;

type WithCallback = Box<dyn Fn(&mlua::Lua) -> Result<(), mlua::Error> + Send + Sync>;

#[derive(Default)]
pub struct Manager {
    with: Option<Arc<WithCallback>>,
}

impl Manager {
    #[allow(unused)]
    pub fn with<F: Fn(&mlua::Lua) -> Result<(), mlua::Error> + Send + Sync + 'static>(
        mut self,
        with: F,
    ) -> Self {
        self.with = Some(Arc::new(Box::new(with)));
        self
    }
}

#[async_trait]
impl managed::Manager for Manager {
    type Type = mlua::Lua;
    type Error = mlua::Error;

    async fn create(&self) -> Result<Self::Type, mlua::Error> {
        let vm = mlua::Lua::new();

        if let Some(with) = &self.with {
            let with = with.clone();
            with(&vm)?;
        }
        Ok(vm)
    }

    async fn recycle(&self, _: &mut Self::Type) -> managed::RecycleResult<Self::Error> {
        Ok(())
    }
}

pub type Pool = managed::Pool<Manager>;
