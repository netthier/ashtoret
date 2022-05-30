use crate::keycodes::Keycode;
use alloc::vec::Vec;

type Layer<const CS: usize, const RS: usize, T> = [[Keycode<T>; CS]; RS];

pub struct Layout<const CS: usize, const RS: usize, T: Clone> {
    layers: Vec<Layer<CS, RS, T>>,
}

impl<const CS: usize, const RS: usize, T: Clone> Layout<CS, RS, T> {
    pub fn new() -> Layout<CS, RS, T> {
        Self { layers: Vec::new() }
    }

    pub fn push_layers(&mut self, layers: impl IntoIterator<Item = Layer<CS, RS, T>>) {
        self.layers.extend(layers.into_iter());
    }

    pub fn get_kc(&self, layer: usize, row: usize, col: usize) -> Option<Keycode<T>> {
        self.layers.get(layer)?.get(row)?.get(col).cloned()
    }
}
