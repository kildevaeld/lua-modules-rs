use async_channel as mpsc;
use std::{
    any::Any,
    future::Future,
    pin::Pin,
    thread::{spawn, JoinHandle},
};

trait Callback {
    fn call<'a>(
        self: Box<Self>,
        vm: &'a mlua::Lua,
    ) -> Pin<Box<dyn Future<Output = Result<Box<dyn Any + Send + 'static>, mlua::Error>> + 'a>>;
}

impl<T, E, U> Callback for T
where
    T: FnOnce(&mlua::Lua) -> U + 'static,
    U: Future<Output = mlua::Result<E>>,
    E: Send + 'static,
{
    fn call<'a>(
        self: Box<Self>,
        vm: &'a mlua::Lua,
    ) -> Pin<Box<dyn Future<Output = Result<Box<dyn Any + Send + 'static>, mlua::Error>> + 'a>>
    {
        Box::pin(async move {
            let result = (self)(vm).await?;
            Ok(Box::new(result) as Box<dyn Any + Send>)
        })
    }
}

pub struct Request {
    callback: Box<dyn Callback + Send>,
    returns: oneshot::Sender<Result<Box<dyn Any + Send>, mlua::Error>>,
}

pub struct Worker {
    sx: Option<mpsc::Sender<Request>>,
    handle: Option<JoinHandle<()>>,
}

impl Worker {
    #[cfg(feature = "tokio")]
    fn create_handle<F, U>(
        rx: mpsc::Receiver<Request>,
        ready: oneshot::Sender<mlua::Result<()>>,
        init: F,
    ) -> Result<JoinHandle<()>, mlua::Error>
    where
        F: FnOnce() -> U + Send + 'static,
        U: Future<Output = mlua::Result<mlua::Lua>>,
    {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .map_err(mlua::Error::external)?;

        let handle = spawn(move || {
            let local = tokio::task::LocalSet::new();

            local.spawn_local(async move {
                let vm = match init().await {
                    Ok(ret) => {
                        ready.send(Ok(())).ok();
                        ret
                    }
                    Err(err) => {
                        ready.send(Err(err)).ok();
                        return;
                    }
                };

                let vm = std::rc::Rc::new(vm);

                while let Ok(next) = rx.recv().await {
                    let vm = vm.clone();

                    tokio::task::spawn_local(async move {
                        let ret = next.callback.call(&vm).await;
                        next.returns.send(ret).ok();
                    });
                }
            });

            rt.block_on(local);
        });

        Ok(handle)
    }

    #[cfg(not(feature = "tokio"))]
    fn create_handle<F, U>(
        rx: mpsc::Receiver<Request>,
        ready: oneshot::Sender<mlua::Result<()>>,
        init: F,
    ) -> Result<JoinHandle<()>, mlua::Error>
    where
        F: FnOnce() -> U + Send + 'static,
        U: Future<Output = mlua::Result<mlua::Lua>>,
    {
        let handle = spawn(move || {
            futures_lite::future::block_on(async move {
                let vm = match init().await {
                    Ok(ret) => {
                        ready.send(Ok(())).ok();
                        ret
                    }
                    Err(err) => {
                        ready.send(Err(err)).ok();
                        return;
                    }
                };

                while let Ok(next) = rx.recv().await {
                    let ret = next.callback.call(&vm).await;
                    next.returns.send(ret).ok();
                }
            })
        });

        Ok(handle)
    }

    pub fn create<F, U>(
        init: F,
    ) -> mlua::Result<(
        oneshot::Receiver<mlua::Result<()>>,
        JoinHandle<()>,
        mpsc::Sender<Request>,
    )>
    where
        F: FnOnce() -> U + Send + 'static,
        U: Future<Output = mlua::Result<mlua::Lua>>,
    {
        let (sx, rx) = mpsc::unbounded();

        let (mark_ready, ready) = oneshot::channel();

        let weak = sx.downgrade();

        let handle = Self::create_handle(rx, mark_ready, move || async move {
            let vm = init().await?;

            vm.set_app_data(WeakWorker { sender: weak });

            Ok(vm)
        })?;

        Ok((ready, handle, sx))
    }

    pub async fn new_async<F, U>(init: F) -> Result<Worker, mlua::Error>
    where
        F: FnOnce() -> U + Send + 'static,
        U: Future<Output = mlua::Result<mlua::Lua>>,
    {
        let (ready, handle, sx) = Self::create(init)?;

        ready
            .await
            .map_err(|_| mlua::Error::external("channel closed"))??;

        Ok(Worker {
            sx: Some(sx),
            handle: Some(handle),
        })
    }

    pub fn new<F, U>(init: F) -> Result<Worker, mlua::Error>
    where
        F: FnOnce() -> U + Send + 'static,
        U: Future<Output = mlua::Result<mlua::Lua>>,
    {
        let (ready, handle, sx) = Self::create(init)?;

        ready
            .recv()
            .map_err(|_| mlua::Error::external("channel closed"))??;

        Ok(Worker {
            sx: Some(sx),
            handle: Some(handle),
        })
    }

    pub async fn with_async<F, U, R>(&self, func: F) -> Result<R, mlua::Error>
    where
        F: FnOnce(&mlua::Lua) -> U + Send + 'static,
        U: Future<Output = mlua::Result<R>>,
        R: 'static + Send,
    {
        let (sx, rx) = oneshot::channel();

        let callback = Box::new(func) as Box<dyn Callback + Send>;

        self.send_async(Request {
            callback,
            returns: sx,
        })
        .await;

        let ret = rx.await.unwrap()?;

        if let Ok(ret) = ret.downcast::<R>() {
            Ok(*ret)
        } else {
            Err(mlua::Error::external("could not convert type"))
        }
    }

    pub fn with<F, U, R>(&self, func: F) -> Result<R, mlua::Error>
    where
        F: FnOnce(&mlua::Lua) -> U + Send + 'static,
        U: Future<Output = mlua::Result<R>>,
        R: 'static + Send,
    {
        let (sx, rx) = oneshot::channel();

        let callback = Box::new(func) as Box<dyn Callback + Send>;

        self.send(Request {
            callback,
            returns: sx,
        });

        let ret = rx.recv().unwrap()?;
        if let Ok(ret) = ret.downcast::<R>() {
            Ok(*ret)
        } else {
            Err(mlua::Error::external("could not convert type"))
        }
    }

    async fn send_async(&self, msg: Request) {
        if let Some(sx) = &self.sx {
            sx.send(msg).await.ok();
        }
    }

    fn send(&self, msg: Request) {
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
    sender: mpsc::WeakSender<Request>,
}

impl WeakWorker {
    pub async fn with_async<F, U, R>(&self, func: F) -> Result<R, mlua::Error>
    where
        F: FnOnce(&mlua::Lua) -> U + Send + 'static,
        U: Future<Output = mlua::Result<R>>,
        R: 'static + Send,
    {
        let (sx, rx) = oneshot::channel();

        let callback = Box::new(func) as Box<dyn Callback + Send>;

        self.send_async(Request {
            callback,
            returns: sx,
        })
        .await?;

        let ret = rx.await.unwrap()?;

        if let Ok(ret) = ret.downcast::<R>() {
            Ok(*ret)
        } else {
            Err(mlua::Error::external("could not convert type"))
        }
    }

    pub fn with<F, U, R>(&self, func: F) -> Result<R, mlua::Error>
    where
        F: FnOnce(&mlua::Lua) -> U + Send + 'static,
        U: Future<Output = mlua::Result<R>>,
        R: 'static + Send,
    {
        let (sx, rx) = oneshot::channel();

        let callback = Box::new(func) as Box<dyn Callback + Send>;

        self.send(Request {
            callback,
            returns: sx,
        })?;

        let ret = rx.recv().unwrap()?;
        if let Ok(ret) = ret.downcast::<R>() {
            Ok(*ret)
        } else {
            Err(mlua::Error::external("could not convert type"))
        }
    }

    async fn send_async(&self, msg: Request) -> mlua::Result<()> {
        if let Some(sx) = self.sender.upgrade() {
            sx.send(msg)
                .await
                .map_err(|_| mlua::Error::external("channel closed"))?;
            Ok(())
        } else {
            Err(mlua::Error::external("worker closed"))
        }
    }

    fn send(&self, msg: Request) -> mlua::Result<()> {
        if let Some(sx) = self.sender.upgrade() {
            sx.send_blocking(msg)
                .map_err(|_| mlua::Error::external("channel closed"))?;
            Ok(())
        } else {
            Err(mlua::Error::external("worker closed"))
        }
    }
}
