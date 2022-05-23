#![no_main]
#![no_std]
#![feature(let_chains)]
#![feature(alloc_error_handler)]

#[macro_export]
macro_rules! outputs {
    ($($pin:expr),* $(,)?) => {{
        use embassy_stm32::gpio::{Level, Output, Speed, Pin};

        [$(
            Output::new($pin.degrade(), Level::Low, Speed::High),
        )*]
    }
}}

#[macro_export]
macro_rules! inputs {
    ($($pin:expr),* $(,)?) => {{
        use embassy_stm32::gpio::{Input, Pull, Pin};

        [$(
            Input::new($pin.degrade(), Pull::Down),
        )*]
    }
}}

mod drivers;
pub mod keyboards;

use alloc_cortex_m::CortexMHeap;
use core::alloc::Layout;
use defmt_rtt as _; // global logger

use panic_probe as _;

#[global_allocator]
static ALLOCATOR: CortexMHeap = CortexMHeap::empty();

// same panicking *behavior* as `panic-probe` but doesn't print a panic message
// this prevents the panic message being printed *twice* when `defmt::panic` is invoked
#[defmt::panic_handler]
fn panic() -> ! {
    cortex_m::asm::udf()
}

#[alloc_error_handler]
fn oom(_: Layout) -> ! {
    cortex_m::asm::udf()
}

/// Terminates the application and makes `probe-run` exit with exit-code = 0
pub fn exit() -> ! {
    loop {
        cortex_m::asm::bkpt();
    }
}

// defmt-test 0.3.0 has the limitation that this `#[tests]` attribute can only be used
// once within a crate. the module can be in any file but there can only be at most
// one `#[tests]` module in this library crate
#[cfg(test)]
#[defmt_test::tests]
mod unit_tests {
    use defmt::assert;

    #[test]
    fn it_works() {
        assert!(true)
    }
}
