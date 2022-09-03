use alloc::sync::Arc;
use core::future::Future;

use ashtoret::drivers::{
    matrix::{MatrixArray, MatrixScanner},
    mcp23018::{Mcp23018, Port},
};
use defmt::error;
use embassy_stm32::gpio::{AnyPin, Input, Output};
use embassy_sync::{blocking_mutex::raw::ThreadModeRawMutex, mutex::Mutex};
use embassy_time::{block_for, Duration};
use embedded_hal_async::i2c::I2c;

pub struct MoonlanderMatrix<I2C> {
    pub rows: [Output<'static, AnyPin>; 6],
    pub cols: [Input<'static, AnyPin>; 7],
    pub ext: Mcp23018<I2C>,
    pub ext_init: bool,
    pub leds: Arc<Mutex<ThreadModeRawMutex, [bool; 3]>>,
}

impl<I2C: 'static, E> MoonlanderMatrix<I2C>
where
    I2C: I2c<Error = E>,
{
    async fn init_ext(&mut self) -> Result<(), E> {
        self.ext
            .config_port(Port::A, 0b00000000, 0b10000000)
            .await?;
        self.ext.config_port(Port::B, 0b00111111, 0b11111111).await
    }

    async fn scan_ext(&mut self, matrix: &mut MatrixArray<7, 14>) -> Result<bool, E> {
        let mut has_changed = false;
        let led_lock = self.leds.lock().await;
        let led_mask = [
            (!led_lock[2] as u8) << 7,
            (!led_lock[1] as u8) << 6 | (!led_lock[0] as u8) << 7,
        ];
        for (y, row) in (0..6).zip(matrix.iter_mut().skip(6)) {
            self.ext
                .set_all([0x01 << y | led_mask[0], led_mask[1]])
                .await?;
            let data = self.ext.read_port(Port::B).await?;
            for (x, col) in row.iter_mut().enumerate() {
                let val = data & (0x01 << x) == 0x01 << x;
                if *col != val {
                    has_changed = true;
                    *col = val;
                }
            }
        }
        Ok(has_changed)
    }
}

impl<I2C: 'static, E> MatrixScanner<7, 14> for MoonlanderMatrix<I2C>
where
    I2C: I2c<Error = E>,
{
    type ScanFuture<'a> = impl Future<Output = bool>;

    fn scan<'a>(&'a mut self, matrix: &'a mut MatrixArray<7, 14>) -> Self::ScanFuture<'a> {
        async move {
            let mut has_changed = false;
            for (y, row_pin) in self.rows.iter_mut().enumerate() {
                row_pin.set_high();
                // Note: No delay is necessary here, signal propagation takes 1ns, MCU period is 14ns @ 72MHz
                for (x, col) in self.cols.iter().enumerate() {
                    let val = col.is_high();
                    if matrix[y][x] != val {
                        has_changed = true;
                        matrix[y][x] = val;
                    }
                }
                row_pin.set_low();
            }

            if !self.ext_init && self.init_ext().await.is_ok() {
                self.ext_init = true;
            }

            if self.ext_init {
                let res = self.scan_ext(matrix).await;
                match res {
                    Ok(e) => has_changed |= e,
                    Err(_) => {
                        self.ext_init = false;
                        error!("Failed to scan right half of Moonlander despite it being initialized. This error is expected if you just unplugged it.");
                    }
                }
            }
            has_changed
        }
    }
}
