#[cfg(feature = "pool")]
mod async_pool;
mod callback;
mod worker;
pub use self::{callback::*, worker::Worker};

#[cfg(feature = "pool")]
pub mod pool {
    pub use super::async_pool::*;
}

pub mod unsend;
