use crate::drivers::mcp23018::Mcp23018;
use embassy_stm32::gpio::{AnyPin, Input, Output};
use embassy_stm32::peripherals::I2C1;

pub struct MatrixScanner {
    pub row_pins: [Output<'static, AnyPin>; 6],
    pub col_pins: [Input<'static, AnyPin>; 7],
    pub right_side: Mcp23018<I2C1>,
}

pub fn scan_matrix(pins: &MatrixScanner) -> [[bool; super::MATRIX_COLS]; super::MATRIX_ROWS] {
    todo!()
}
