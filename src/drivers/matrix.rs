use core::future::Future;

use embassy_stm32::gpio::{AnyPin, Input, Output};
use embassy_time::{Duration, Timer};

pub type MatrixArray<const CS: usize, const RS: usize> = [[bool; CS]; RS];

// Note: Common scanning types like simple row2col/col2row should be defined here
pub trait MatrixScanner<const CS: usize, const RS: usize> {
    type ScanFuture<'a>: Future<Output = bool>
    where
        Self: 'a;
    fn scan<'a>(&'a mut self, matrix: &'a mut MatrixArray<CS, RS>) -> Self::ScanFuture<'a>;
}

#[derive(Copy, Clone, Default, Debug, defmt::Format)]
pub enum KeyEvent {
    Pressed(u16),
    Released(u16),
    #[default]
    Nop,
}

// TODO: Bitstuffing
pub struct Matrix<const CS: usize, const RS: usize, SC> {
    state: MatrixArray<CS, RS>,
    prev_state: MatrixArray<CS, RS>,
    scanner: SC,
    debounce: Option<Duration>,
}

impl<const CS: usize, const RS: usize, SC> Matrix<{ CS }, { RS }, SC>
where
    SC: MatrixScanner<CS, RS>,
{
    pub fn new(scanner: SC) -> Self {
        Self {
            state: [[false; CS]; RS],
            prev_state: [[false; CS]; RS],
            scanner,
            debounce: None,
        }
    }

    pub fn with_debounce(mut self, time: Duration) -> Self {
        self.debounce = Some(time);
        self
    }

    async fn raw_update(&mut self) -> Option<tinyvec::TinyVec<[KeyEvent; 6]>> {
        core::mem::swap(&mut self.prev_state, &mut self.state);
        if self.scanner.scan(&mut self.state).await {
            let mut vec = tinyvec::TinyVec::new();

            for (idx, (key, prev_key)) in self
                .state
                .iter()
                .flatten()
                .zip(self.prev_state.iter().flatten())
                .enumerate()
            {
                if let Some(event) = if *key && !prev_key {
                    Some(KeyEvent::Pressed(idx as u16))
                } else if !key && *prev_key {
                    Some(KeyEvent::Released(idx as u16))
                } else {
                    None
                } {
                    vec.push(event)
                }
            }

            Some(vec)
        } else {
            None
        }
    }

    pub async fn update(&mut self) -> Option<tinyvec::TinyVec<[KeyEvent; 6]>> {
        let changes = self.raw_update().await?;
        if let Some(debounce) = self.debounce {
            Timer::after(debounce).await;
            if self.raw_update().await.is_none() {
                Some(changes)
            } else {
                None
            }
        } else {
            Some(changes)
        }
    }
}
