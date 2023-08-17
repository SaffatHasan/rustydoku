mod display;
mod gui;
mod solver;
mod sudoku;
mod sudoku_error;
use crate::sudoku::Sudoku;

fn main() -> Result<(), eframe::Error> {
    gui::run()
}
