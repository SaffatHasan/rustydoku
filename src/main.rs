mod sudoku;

use sudoku::Sudoku;
mod display;

fn main() {
    let values: [[u8; 9]; 9] = [[1, 2, 3, 4, 5, 6, 7, 8, 9]; 9];
    let mut doku: Sudoku = Sudoku::new_with_values(values);
    println!("{}", doku);
    if doku.verify() {
        println!("The above sudoku board is valid.");
    } else {
        println!("The above sudoku board is not valid.");
    }
    doku.verify_then_set(0, 0, 1);
}
