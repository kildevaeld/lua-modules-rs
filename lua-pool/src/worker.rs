use async_channel as mpsc;
use std::{
    any::Any,
    thread::{spawn, JoinHandle},
};

use crate::{LuaCallback, Return};

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
    sx: Option<mpsc::Sender<Msg>>,
    handle: Option<JoinHandle<()>>,
}

// impl Default for Worker {
//     fn default() -> Self {
//         Worker::new(|| Ok(mlua::Lua::new()))
//     }
// }

impl Worker {
    #[cfg(feature = "tokio")]
    pub async fn new<F>(init: F) -> Result<Worker, mlua::Error>
    where
        F: FnOnce() -> Result<mlua::Lua, mlua::Error> + Send + 'static,
    {
        use std::rc::Rc;

        let (sx, rx) = mpsc::unbounded();

        let (mark_ready, ready) = oneshot::channel();

        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .map_err(mlua::Error::external)?;

        let handle = spawn(move || {
            let local = tokio::task::LocalSet::new();

            local.spawn_local(async move {
                let vm = match init() {
                    Ok(ret) => {
                        mark_ready.send(Ok(())).ok();
                        ret
                    }
                    Err(err) => {
                        mark_ready.send(Err(err)).ok();
                        return;
                    }
                };

                let vm = Rc::new(vm);

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

        ready.await.expect("ready channel")?;

        Ok(Worker {
            sx: Some(sx),
            handle: Some(handle),
        })
    }

    #[cfg(all(feature = "async", not(feature = "tokio")))]
    pub async fn new<F>(init: F) -> Result<Worker, mlua::Error>
    where
        F: FnOnce() -> Result<mlua::Lua, mlua::Error> + Send + 'static,
    {
        let (sx, rx) = mpsc::bounded(1);

        let (mark_ready, ready) = oneshot::channel();

        let handle = spawn(move || {
            futures_lite::future::block_on(async move {
                let vm = match init() {
                    Ok(ret) => {
                        mark_ready.send(Ok(())).ok();
                        ret
                    }
                    Err(err) => {
                        mark_ready.send(Err(err)).ok();
                        return;
                    }
                };

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
        let (sx, rx) = mpsc::bounded(1);
        let (mark_ready, ready) = oneshot::channel();

        let handle = spawn(move || {
            let vm = match init() {
                Ok(ret) => {
                    mark_ready.send(Ok(())).ok();
                    ret
                }
                Err(err) => {
                    mark_ready.send(Err((err))).ok();
                    return;
                }
            };

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
