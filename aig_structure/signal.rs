#![allow(warnings)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)] // tells the compiler that it should use automatically standard functions for this struct


pub struct Signal {
    pub index: usize,
    pub inverted: bool
}

impl Signal {
    pub fn new(index: usize, inverted: bool) -> Self {
        Signal {index, inverted}
    }

    pub fn invert(self) -> Self {
        Signal::new(self.index, !self.inverted)
    }

}