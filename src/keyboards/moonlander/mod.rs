mod matrix;

use crate::drivers::matrix::{Matrix, ScanKind};
use crate::drivers::mcp23018::Mcp23018;
use crate::keyboards::moonlander::matrix::MatrixScanner;
use crate::keyboards::Keyboard;
use embassy_stm32::Peripherals;

pub const MATRIX_COLS: usize = 7;
pub const MATRIX_ROWS: usize = 12;

pub struct Moonlander {
    matrix: Matrix<MATRIX_COLS, MATRIX_ROWS, MatrixScanner>,
}

impl Keyboard for Moonlander {
    fn init(p: Peripherals) -> Self {
        let matrix = Matrix::new(ScanKind::Custom(MatrixScanner {
            row_pins: outputs![p.PB10, p.PB11, p.PB12, p.PB13, p.PB14, p.PB15],
            col_pins: inputs![p.PA0, p.PA1, p.PA2, p.PA3, p.PA6, p.PA7, p.PB0,],
            right_side: Mcp23018::new(),
        }))
        .with_custom_scan(|scanner| {
            if let ScanKind::Custom(scanner) = scanner {
                matrix::scan_matrix(scanner)
            } else {
                unreachable!()
            }
        });

        Self { matrix }
    }

    fn run(self) -> ! {
        cortex_m::asm::udf()
    }
}
