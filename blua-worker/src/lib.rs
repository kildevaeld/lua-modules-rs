mod worker;

pub use self::worker::{WeakWorker, Worker};

mod sealed {
    use crate::Worker;

    pub trait Sealed {}
    impl Sealed for mlua::Lua {}
    impl Sealed for Worker {}
}

pub trait LuaExt: sealed::Sealed {
    fn worker(&self) -> mlua::Result<WeakWorker>;
}

impl LuaExt for mlua::Lua {
    fn worker(&self) -> mlua::Result<WeakWorker> {
        self.app_data_ref::<WeakWorker>()
            .map(|data| data.clone())
            .ok_or_else(|| mlua::Error::external("weak worker"))
    }
}

impl LuaExt for Worker {
    fn worker(&self) -> mlua::Result<WeakWorker> {
        Ok(self.downgrade())
    }
}
