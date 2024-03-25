use crate::callable::ItemId;
use crate::{LuaCallback, Return};
use async_channel as mpsc;
use mlua::RegistryKey;
use std::collections::BTreeMap;
use std::{
    any::Any,
    sync::{Arc, Weak},
    thread::{spawn, JoinHandle},
};

enum Msg {
    With {
        with: Box<dyn LuaCallback<()> + Send>,
        returns: oneshot::Sender<Result<(), mlua::Error>>,
    },
    WithReturn {
        with: Box<dyn LuaCallback<Box<dyn Any + Send>> + Send>,
        returns: oneshot::Sender<Result<Box<dyn Any + Send>, mlua::Error>>,
    },
}

pub struct Worker {
    sx: Option<Arc<mpsc::Sender<Msg>>>,
    handle: Option<JoinHandle<()>>,
}

impl Worker {
    #[cfg(feature = "tokio")]
    async fn create_handle<F>(
        sx: &Arc<mpsc::Sender<Msg>>,
        rx: mpsc::Receiver<Msg>,
        ready: oneshot::Sender<mlua::Result<()>>,
        init: F,
    ) -> Result<JoinHandle<()>, mlua::Error>
    where
        F: FnOnce() -> Result<mlua::Lua, mlua::Error> + Send + 'static,
    {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .map_err(mlua::Error::external)?;

        let weak = Arc::downgrade(sx);

        let handle = spawn(move || {
            let local = tokio::task::LocalSet::new();

            local.spawn_local(async move {
                let vm = match init() {
                    Ok(ret) => {
                        ready.send(Ok(())).ok();
                        ret
                    }
                    Err(err) => {
                        ready.send(Err(err)).ok();
                        return;
                    }
                };

                vm.set_app_data(WeakWorker { sender: weak });

                let vm = std::rc::Rc::new(vm);

                while let Ok(next) = rx.recv().await {
                    let vm = vm.clone();

                    tokio::task::spawn_local(async move {
                        match next {
                            Msg::With { with, returns } => {
                                let ret = with.call(&vm).await;
                                returns.send(ret).ok();
                            }
                            Msg::WithReturn { with, returns } => {
                                let ret = with.call(&vm).await;
                                returns.send(ret).ok();
                            }
                        }
                    });
                }
            });

            rt.block_on(local);
        });

        Ok(handle)
    }

    #[cfg(all(feature = "async", not(feature = "tokio")))]
    async fn create_handle<F>(
        sx: &Arc<mpsc::Sender<Msg>>,
        rx: mpsc::Receiver<Msg>,
        ready: oneshot::Sender<mlua::Result<()>>,
        init: F,
    ) -> Result<JoinHandle<()>, mlua::Error>
    where
        F: FnOnce() -> Result<mlua::Lua, mlua::Error> + Send + 'static,
    {
        let weak = Arc::downgrade(sx);

        let handle = spawn(move || {
            futures_lite::future::block_on(async move {
                let vm = match init() {
                    Ok(ret) => {
                        ready.send(Ok(())).ok();
                        ret
                    }
                    Err(err) => {
                        ready.send(Err(err)).ok();
                        return;
                    }
                };

                vm.set_app_data(WeakWorker { sender: weak });

                while let Ok(next) = rx.recv().await {
                    match next {
                        Msg::With { with, returns } => {
                            let ret = with.call(&vm).await;
                            returns.send(ret).ok();
                        }
                        Msg::WithReturn { with, returns } => {
                            let ret = with.call(&vm).await;
                            returns.send(ret).ok();
                        }
                    }
                }
            })
        });

        Ok(handle)
    }

    #[cfg(not(feature = "async"))]
    fn create_handle<F>(
        sx: &Arc<mpsc::Sender<Msg>>,
        rx: mpsc::Receiver<Msg>,
        ready: oneshot::Sender<mlua::Result<()>>,
        init: F,
    ) -> Result<JoinHandle<()>, mlua::Error>
    where
        F: FnOnce() -> Result<mlua::Lua, mlua::Error> + Send + 'static,
    {
        let weak = Arc::downgrade(sx);

        let handle = spawn(move || {
            let vm = match init() {
                Ok(ret) => {
                    ready.send(Ok(())).ok();
                    ret
                }
                Err(err) => {
                    ready.send(Err(err)).ok();
                    return;
                }
            };

            vm.set_app_data(WeakWorker { sender: weak });

            while let Ok(next) = rx.recv_blocking() {
                match next {
                    Msg::With { with, returns } => {
                        let ret = with.call(&vm);
                        returns.send(ret).ok();
                    }
                    Msg::WithReturn { with, returns } => {
                        let ret = with.call(&vm);
                        returns.send(ret).ok();
                    }
                }
            }
        });

        Ok(handle)
    }

    #[cfg(feature = "async")]
    pub async fn new<F>(init: F) -> Result<Worker, mlua::Error>
    where
        F: FnOnce() -> Result<mlua::Lua, mlua::Error> + Send + 'static,
    {
        let (sx, rx) = mpsc::unbounded();
        let sx = Arc::new(sx);

        let (mark_ready, ready) = oneshot::channel();

        let handle = Self::create_handle(&sx, rx, mark_ready, move || {
            let vm = init()?;
            vm.set_app_data(BTreeMap::<ItemId, RegistryKey>::new());
            Ok(vm)
        })
        .await?;

        ready.await.expect("ready channel")?;

        Ok(Worker {
            sx: Some(sx),
            handle: Some(handle),
        })
    }

    #[cfg(not(feature = "async"))]
    pub fn new<F>(init: F) -> Result<Worker, mlua::Error>
    where
        F: FnOnce() -> Result<mlua::Lua, mlua::Error> + Send + 'static,
    {
        let (sx, rx) = mpsc::unbounded();
        let sx = Arc::new(sx);

        let (mark_ready, ready) = oneshot::channel();

        let handle = Self::create_handle(&sx, rx, mark_ready, move || {
            let vm = init()?;
            vm.set_app_data(BTreeMap::<ItemId, RegistryKey>::new());
            Ok(vm)
        })?;

        ready
            .recv_timeout(std::time::Duration::from_millis(500))
            .expect("ready channel")?;

        Ok(Worker {
            sx: Some(sx),
            handle: Some(handle),
        })
    }

    pub async fn with_async<F>(&self, func: F) -> Result<(), mlua::Error>
    where
        F: LuaCallback<()> + Send + 'static,
    {
        let (sx, rx) = oneshot::channel();
        self.send_async(Msg::With {
            with: Box::new(func),
            returns: sx,
        })
        .await;

        rx.await.unwrap()
    }

    pub async fn with_async_ret<F, R>(&self, func: F) -> Result<R, mlua::Error>
    where
        F: LuaCallback<R> + Send + 'static,
        R: 'static + Send,
    {
        let (sx, rx) = oneshot::channel();

        let cb = Return::new(func);

        let with = Box::new(cb) as Box<dyn LuaCallback<Box<dyn Any + Send>> + Send>;

        self.send_async(Msg::WithReturn { with, returns: sx }).await;

        let ret = rx.await.unwrap()?;

        if let Ok(ret) = ret.downcast::<R>() {
            Ok(*ret)
        } else {
            Err(mlua::Error::external("could not convert type"))
        }
    }

    pub fn with<F>(&self, func: F) -> Result<(), mlua::Error>
    where
        F: LuaCallback<()> + Send + 'static,
    {
        let (sx, rx) = oneshot::channel();
        self.send(Msg::With {
            with: Box::new(func),
            returns: sx,
        });

        rx.recv().unwrap()
    }

    pub fn with_ret<F, R>(&self, func: F) -> Result<R, mlua::Error>
    where
        F: LuaCallback<R> + Send + 'static,
        R: 'static + Send,
    {
        let (sx, rx) = oneshot::channel();

        let cb = Return::new(func);

        let with = Box::new(cb) as Box<dyn LuaCallback<Box<dyn Any + Send>> + Send>;

        self.send(Msg::WithReturn { with, returns: sx });

        let ret = rx.recv().unwrap()?;

        if let Ok(ret) = ret.downcast::<R>() {
            Ok(*ret)
        } else {
            Err(mlua::Error::external("could not convert type"))
        }
    }

    async fn send_async(&self, msg: Msg) {
        if let Some(sx) = &self.sx {
            sx.send(msg).await.ok();
        }
    }

    fn send(&self, msg: Msg) {
        if let Some(sx) = &self.sx {
            sx.send_blocking(msg).expect("send");
        }
    }
}

impl Drop for Worker {
    fn drop(&mut self) {
        drop(self.sx.take().unwrap());
        drop(self.handle.take().unwrap().join());
    }
}

#[derive(Clone)]
pub struct WeakWorker {
    sender: Weak<mpsc::Sender<Msg>>,
}

impl WeakWorker {
    pub async fn with_async<F>(&self, func: F) -> Result<(), mlua::Error>
    where
        F: LuaCallback<()> + Send + 'static,
    {
        let (sx, rx) = oneshot::channel();
        self.send_async(Msg::With {
            with: Box::new(func),
            returns: sx,
        })
        .await?;

        rx.await.unwrap()
    }

    pub async fn with_async_ret<F, R>(&self, func: F) -> Result<R, mlua::Error>
    where
        F: LuaCallback<R> + Send + 'static,
        R: 'static + Send,
    {
        let (sx, rx) = oneshot::channel();

        let cb = Return::new(func);

        let with = Box::new(cb) as Box<dyn LuaCallback<Box<dyn Any + Send>> + Send>;

        self.send_async(Msg::WithReturn { with, returns: sx })
            .await?;

        let ret = rx.await.unwrap()?;

        if let Ok(ret) = ret.downcast::<R>() {
            Ok(*ret)
        } else {
            Err(mlua::Error::external("could not convert type"))
        }
    }

    pub fn with<F>(&self, func: F) -> Result<(), mlua::Error>
    where
        F: LuaCallback<()> + Send + 'static,
    {
        let (sx, rx) = oneshot::channel();
        self.send(Msg::With {
            with: Box::new(func),
            returns: sx,
        })?;

        rx.recv().unwrap()
    }

    pub fn with_ret<F, R>(&self, func: F) -> Result<R, mlua::Error>
    where
        F: LuaCallback<R> + Send + 'static,
        R: 'static + Send,
    {
        let (sx, rx) = oneshot::channel();

        let cb = Return::new(func);

        let with = Box::new(cb) as Box<dyn LuaCallback<Box<dyn Any + Send>> + Send>;

        self.send(Msg::WithReturn { with, returns: sx })?;

        let ret = rx.recv().unwrap()?;

        if let Ok(ret) = ret.downcast::<R>() {
            Ok(*ret)
        } else {
            Err(mlua::Error::external("could not convert type"))
        }
    }

    async fn send_async(&self, msg: Msg) -> mlua::Result<()> {
        if let Some(sx) = self.sender.upgrade() {
            sx.send(msg)
                .await
                .map_err(|_| mlua::Error::external("channel closed"))?;
            Ok(())
        } else {
            Err(mlua::Error::external("worker closed"))
        }
    }

    fn send(&self, msg: Msg) -> mlua::Result<()> {
        if let Some(sx) = self.sender.upgrade() {
            sx.send_blocking(msg)
                .map_err(|_| mlua::Error::external("channel closed"))?;
            Ok(())
        } else {
            Err(mlua::Error::external("worker closed"))
        }
    }
}

mod sealed {
    pub trait Sealed {}

    impl Sealed for mlua::Lua {}
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
