use alloc::sync::Arc;

use embassy_stm32::gpio::{AnyPin, Output};
use embassy_sync::{blocking_mutex::raw::ThreadModeRawMutex, mutex::Mutex};

pub struct Leds {
    pub left: [Output<'static, AnyPin>; 3],
    pub right: Arc<Mutex<ThreadModeRawMutex, [bool; 3]>>,
}

impl Leds {
    pub async fn set(&mut self, idx: usize, val: bool) {
        match idx {
            0..=2 => {
                let pin = &mut self.left[idx];
                if val {
                    pin.set_high();
                } else {
                    pin.set_low();
                }
            }
            3..=5 => {
                self.right.lock().await[idx - 3] = val;
            }
            _ => {}
        }
    }

    pub async fn set_all(&mut self, val: u8) {
        let mut lock = self.right.lock().await;
        for (idx, pin) in self.left.iter_mut().enumerate() {
            if val & (0x01 << idx) == (0x01 << idx) {
                pin.set_high();
            } else {
                pin.set_low();
            }

            let sh = idx + 3;
            lock[idx] = val & (0x01 << sh) == (0x01 << sh);
        }
    }

    pub async fn clear(&mut self) {
        self.set_all(0).await;
    }
}
