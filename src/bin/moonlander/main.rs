#![no_main]
#![no_std]
#![feature(type_alias_impl_trait)]
#![feature(generic_associated_types)]

mod leds;
mod matrix;

extern crate alloc;

use alloc::{sync::Arc, vec::Vec};

use ashtoret as _; // global logger + panicking-behavior + memory layout
use ashtoret::{
    drivers::{matrix::Matrix, mcp23018::Mcp23018},
    init_alloc, inputs, outputs,
};
use embassy_embedded_hal::shared_bus::asynch::i2c::I2cDevice;
use embassy_executor::{Spawner, _export::StaticCell};
use embassy_stm32::{
    gpio::{AnyPin, Output},
    i2c::I2c,
    interrupt,
    peripherals::{DMA1_CH6, DMA1_CH7, I2C1},
    time::hz,
    Peripherals,
};
use embassy_sync::{blocking_mutex::raw::ThreadModeRawMutex, mutex::Mutex};
use embassy_time::{Duration, Timer};

use crate::{leds::Leds, matrix::MoonlanderMatrix};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    init_alloc();
    let p = embassy_stm32::init(Default::default());
    let mut leds = Leds {
        left: outputs![p.PB5, p.PB4, p.PB3],
        right: Arc::new(Mutex::new([false; 3])),
    };

    static I2C_BUS: StaticCell<Mutex<ThreadModeRawMutex, I2c<I2C1, DMA1_CH6, DMA1_CH7>>> =
        StaticCell::new();
    let i2c_bus = I2c::new(
        p.I2C1,
        p.PB6,
        p.PB7,
        interrupt::take!(I2C1_EV),
        p.DMA1_CH6,
        p.DMA1_CH7,
        hz(100000),
        Default::default(),
    );
    let i2c_bus = I2C_BUS.init(Mutex::new(i2c_bus));
    let i2c_dev1 = I2cDevice::new(i2c_bus);
    let mcp23018 = Mcp23018::new(i2c_dev1);
    let mut matrix = Matrix::<7, 14, _>::new(MoonlanderMatrix {
        rows: outputs![p.PB10, p.PB11, p.PB12, p.PB13, p.PB14, p.PB15],
        cols: inputs![p.PA0, p.PA1, p.PA2, p.PA3, p.PA6, p.PA7, p.PB0],
        ext: mcp23018,
        ext_init: false,
        leds: Arc::clone(&leds.right),
    })
    .with_debounce(Duration::from_millis(5));

    // placeholder, binary counting on both halves
    let mut counter = 0;
    loop {
        leds.set_all(counter).await;
        counter += 1;
        Timer::after(Duration::from_millis(250)).await;
        matrix.update().await;
    }
}
