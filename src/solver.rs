use crate::Sudoku;
use crate::sudoku_error::SudokuError;

// Solve with DFS and return the first legal solution.
fn solve_sudoku(sudoku: &Sudoku) -> Result<Sudoku, SudokuError> {
    if !sudoku.verify() {
        return Err(SudokuError::NoSolution);
    }
    if sudoku.is_solved() {
        return Ok(*sudoku);
    }
    for row in 0..9 {
        for col in 0..9 {
            if sudoku.get(row, col) != 0 {
                continue;
            }
            for val in 1..10 {
                let result: Result<Sudoku, SudokuError> = solve_sudoku(&sudoku.set(row, col, val));
                if result.is_ok() {
                    return result;
                }
            }
        }
    }
    return Err(SudokuError::NoSolution);
}

#[cfg(test)]
mod tests {
    use crate::Sudoku;
    use super::*;

    fn solved_sudoku() -> Sudoku {
        Sudoku::new_with_values([
            [5, 3, 4, 6, 7, 8, 9, 1, 2],
            [6, 7, 2, 1, 9, 5, 3, 4, 8],
            [1, 9, 8, 3, 4, 2, 5, 6, 7],
            [8, 5, 9, 7, 6, 1, 4, 2, 3],
            [4, 2, 6, 8, 5, 3, 7, 9, 1],
            [7, 1, 3, 9, 2, 4, 8, 5, 6],
            [9, 6, 1, 5, 3, 7, 2, 8, 4],
            [2, 8, 7, 4, 1, 9, 6, 3, 5],
            [3, 4, 5, 2, 8, 6, 1, 7, 9],
        ])
    }

    #[test]
    fn test_solve_already_complete_sudoku() {
        let result = solve_sudoku(&solved_sudoku());
        assert_eq!(result.unwrap(), solved_sudoku());
    }

    #[test]
    fn test_solve_one_missing_value() {
        let missing_one = solved_sudoku().set(0, 0, 0);
        let result = solve_sudoku(&missing_one);
        assert_eq!(result.unwrap(), solved_sudoku());
    }

    #[test]
    fn test_solve_one_row() {
        let first_row_zero = &mut solved_sudoku();
        for i in 0..9 {
            *first_row_zero = first_row_zero.set(0,i,0);
        }
        let result = solve_sudoku(&first_row_zero).unwrap();
        assert_eq!(result, solved_sudoku());
    }
}
