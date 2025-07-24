use crate::aig_structure::signal::Signal;

pub struct AndNode {
    pub left_signal: Signal,
    pub right_signal: Signal,
}