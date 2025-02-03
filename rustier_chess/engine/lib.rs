#![allow(dead_code)]

pub mod board;
pub mod bots;
mod magic;
pub mod moves;
pub mod perft;
mod types;
pub mod utils;

#[cfg(feature = "pyo3")]
use {crate::board::Board, pyo3::prelude::*};

#[pyfunction]
#[cfg(feature = "pyo3")]
#[pyo3(name = "abc")]
pub fn print_board_test(_py: Python) {
    let board = Board::default();
    println!("{}", board);
}

#[pymodule]
#[cfg(feature = "pyo3")]
#[pyo3(name = "engine")]
pub fn engine(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(print_board_test, m)?)
}
