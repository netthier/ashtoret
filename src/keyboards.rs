use embassy_stm32::Peripherals;

pub trait Keyboard {
    fn init(p: Peripherals) -> Self;
    fn run(self) -> !;
}

pub mod moonlander;
