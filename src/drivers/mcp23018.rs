use crate::drivers::SharedBus;
use embassy_stm32::i2c::{self, I2c};

pub struct Mcp23018<T: i2c::Instance> {
    i2c: SharedBus<I2c<'static, T>>,
}
impl<T: i2c::Instance> Mcp23018<T> {
    pub fn new() -> Self {
        todo!()
    }
}
