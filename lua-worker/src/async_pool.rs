use crate::worker::Worker;
use deadpool::{async_trait, managed};
use std::sync::Arc;

type WithCallback = Box<dyn Fn(&mlua::Lua) -> Result<(), mlua::Error> + Send + Sync>;

#[derive(Default)]
pub struct Manager {
    with: Option<Arc<WithCallback>>,
}

impl Manager {
    pub fn new<F: Fn(&mlua::Lua) -> Result<(), mlua::Error> + Send + Sync + 'static>(
        mut self,
        with: F,
    ) -> Self {
        self.with = Some(Arc::new(Box::new(with)));
        self
    }
}

#[async_trait]
impl managed::Manager for Manager {
    type Type = Worker;
    type Error = mlua::Error;

    async fn create(&self) -> Result<Self::Type, mlua::Error> {
        let worker = Worker::default();

        tracing::debug!("create lua vm");

        if let Some(with) = &self.with {
            let with = with.clone();
            #[cfg(feature = "async")]
            worker
                .with_async(move |ctx: &mlua::Lua, _table: &mlua::Table| {
                    let ret = with(ctx);
                    async move { ret }
                })
                .await?;
            #[cfg(not(feature = "async"))]
            worker
                .with_async(move |ctx: &mlua::Lua, _table: mlua::Table<'_>| with(ctx))
                .await?;
        }
        Ok(worker)
    }

    async fn recycle(&self, _: &mut Self::Type) -> managed::RecycleResult<Self::Error> {
        Ok(())
    }
}

pub type AsyncPool = managed::Pool<Manager>;
