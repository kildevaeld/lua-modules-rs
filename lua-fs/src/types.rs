#[cfg(feature = "send")]
pub type Lrc<T> = std::sync::Arc<T>;

#[cfg(not(feature = "send"))]
pub type Lrc<T> = std::rc::Rc<T>;

#[cfg(feature = "send")]
pub type Locket<T> = std::sync::Arc<tokio::sync::Mutex<T>>;

#[cfg(not(feature = "send"))]
pub type Locket<T> = std::rc::Rc<std::cell::RefCell<T>>;

pub fn new_lock<T>(item: T) -> Locket<T> {
    #[cfg(feature = "send")]
    let lock = Locket::new(tokio::sync::Mutex::new(item));
    #[cfg(not(feature = "send"))]
    let lock = Locket::new(std::cell::RefCell::new(item));
    lock
}
