use crate::sudoku_error::SudokuError;
use crate::Sudoku;

// Solve with DFS and return the first legal solution.
pub fn solve_sudoku(sudoku: &Sudoku) -> Result<Sudoku, SudokuError> {
    if !sudoku.verify() {
        return Err(SudokuError::NoSolution);
    }
    if sudoku.is_solved() {
        return Ok(*sudoku);
    }
    for row in 0..9 {
        for col in 0..9 {
            if sudoku.get(row, col).is_some() {
                continue;
            }
            for val in 1..10 {
                // Perform local check (fail fast).
                if !sudoku.is_valid_to_set(row, col, val) {
                    continue;
                }
                let result: Result<Sudoku, SudokuError> = solve_sudoku(&sudoku.set(row, col, val));
                if result.is_ok() {
                    return result;
                }
            }
        }
    }
    Err(SudokuError::NoSolution)
}

#[allow(dead_code)]
pub fn solve_sudoku_bfs(sudoku: &Sudoku) -> Result<Sudoku, SudokuError> {
    if !sudoku.verify() {
        return Err(SudokuError::NoSolution);
    }

    if sudoku.is_solved() {
        return Ok(*sudoku);
    }

    let mut queue: Vec<Sudoku> = Vec::new();
    queue.push(*sudoku);

    while let Some(current) = queue.pop() {
        for row in 0..9 {
            for col in 0..9 {
                if current.get(row, col).is_some() {
                    continue;
                }

                for val in 1..10 {
                    // Greedy, local check.
                    if !current.is_valid_to_set(row, col, val) {
                        continue;
                    }
                    let new_sudoku = current.set(row, col, val);

                    if !new_sudoku.verify() {
                        continue;
                    }
                    queue.push(new_sudoku);

                    if new_sudoku.is_solved() {
                        return Ok(new_sudoku);
                    }
                }
            }
        }
    }

    Err(SudokuError::NoSolution)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Sudoku;

    fn solved_sudoku() -> Sudoku {
        Sudoku::new([
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
        let result: Result<Sudoku, SudokuError> = solve_sudoku(&solved_sudoku());
        assert_eq!(result.unwrap(), solved_sudoku());
    }

    #[test]
    fn test_solve_one_missing_value() {
        let missing_one: Sudoku = solved_sudoku().unset(0, 0);
        let result: Result<Sudoku, SudokuError> = solve_sudoku(&missing_one);
        assert_eq!(result.unwrap(), solved_sudoku());
    }

    #[test]
    fn test_solve_one_row() {
        let first_row_zero: &mut Sudoku = &mut solved_sudoku();
        for i in 0..9 {
            *first_row_zero = first_row_zero.unset(0, i);
        }
        let result = solve_sudoku(first_row_zero).unwrap();
        assert_eq!(result, solved_sudoku());
    }

    #[test]
    fn test_solve_one_row_bfs() {
        let first_row_zero: &mut Sudoku = &mut solved_sudoku();
        for i in 0..9 {
            *first_row_zero = first_row_zero.unset(0, i);
        }
        let result: Sudoku = solve_sudoku_bfs(first_row_zero).unwrap();
        assert_eq!(result, solved_sudoku());
    }

    // Takes too long. Skip this.
    // #[test]
    // fn test_solve_empty() {
    //     let result = solve_sudoku_bfs(&Sudoku::default()).unwrap();
    //     assert_eq!(result, solved_sudoku());
    // }
}
