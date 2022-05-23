#![no_main]
#![no_std]
#![feature(type_alias_impl_trait)]

use ashtoret as _; // global logger + panicking-behavior + memory layout

use defmt::*;

use embassy::executor::Spawner;
use embassy::time::{Duration, Timer};
use embassy_stm32::{
    gpio::{Level, Output, Speed},
    Peripherals,
};

use ashtoret::keyboards::{moonlander::Moonlander, Keyboard};

#[embassy::main]
async fn main(_spawner: Spawner, p: Peripherals) {
    let keyboard = Moonlander::init(p);
    keyboard.run()
}
