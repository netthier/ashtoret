use embassy::blocking_mutex::NoopMutex;

pub mod hid;
pub mod matrix;
pub mod mcp23018;

pub type SharedBus<T> = &'static NoopMutex<T>;
