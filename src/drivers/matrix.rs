use embassy::time::{Duration, Timer};
use embassy_stm32::gpio::{AnyPin, Input, Output};

type MatrixArray<const COLS: usize, const ROWS: usize> = [[bool; COLS]; ROWS];

pub enum ScanKind<const COLS: usize, const ROWS: usize, T> {
    Direct([[Input<'static, AnyPin>; COLS]; ROWS]),
    Row2Col {
        row_pins: [Output<'static, AnyPin>; ROWS],
        col_pins: [Input<'static, AnyPin>; COLS],
    },
    Custom(T),
}

#[derive(Copy, Clone, Default, Debug, defmt::Format)]
pub enum KeyEvent {
    Pressed(u16),
    Released(u16),
    #[default]
    Nop,
}

// TODO: Bitstuffing
pub struct Matrix<const COLS: usize, const ROWS: usize, T> {
    state: MatrixArray<COLS, ROWS>,
    prev_state: MatrixArray<COLS, ROWS>,
    pins: ScanKind<COLS, ROWS, T>,
    debounce: Option<Duration>,
    scan_fn: Option<fn(&ScanKind<COLS, ROWS, T>) -> MatrixArray<COLS, ROWS>>,
}

impl<const COLS: usize, const ROWS: usize, T> Matrix<{ COLS }, { ROWS }, T> {
    pub fn new(pins: ScanKind<COLS, ROWS, T>) -> Self {
        Self {
            state: [[false; COLS]; ROWS],
            prev_state: [[false; COLS]; ROWS],
            pins,
            debounce: None,
            scan_fn: None,
        }
    }

    pub fn with_debounce(mut self, time: Duration) -> Self {
        self.debounce = Some(time);
        self
    }

    pub fn with_custom_scan(
        mut self,
        scan_fn: fn(&ScanKind<COLS, ROWS, T>) -> MatrixArray<COLS, ROWS>,
    ) -> Self {
        self.scan_fn = Some(scan_fn);
        self
    }

    fn raw_update(&mut self) -> tinyvec::TinyVec<[KeyEvent; 6]> {
        core::mem::swap(&mut self.prev_state, &mut self.state);
        self.state = if let Some(custom_scan) = self.scan_fn {
            custom_scan(&self.pins)
        } else {
            // Note: Not needed for the Moonlander, as it implements a custom scanning routine
            todo!()
        };

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

        vec
    }

    pub async fn update(&mut self) -> Option<tinyvec::TinyVec<[KeyEvent; 6]>> {
        let changes = self.raw_update();
        if !changes.is_empty() {
            if let Some(debounce) = self.debounce {
                Timer::after(debounce).await;
                if self.raw_update().is_empty() {
                    Some(changes)
                } else {
                    None
                }
            } else {
                Some(changes)
            }
        } else {
            None
        }
    }
}
