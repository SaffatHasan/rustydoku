use std::collections::HashSet;
#[derive(Copy, Clone, Debug)]
pub struct Sudoku {
    data: [[u8; 9]; 9],
}

impl Sudoku {
    pub fn new() -> Sudoku {
        Sudoku { data: [[0; 9]; 9] }
    }

    pub fn new_with_values(values: [[u8; 9]; 9]) -> Sudoku {
        let sudoku = &mut Sudoku::new();
        for row in 0..9 {
            for col in 0..9 {
                sudoku.set(row, col, values[row][col]);
            }
        }
        *sudoku
    }

    pub fn verify_then_set(&mut self, row: usize, col: usize, val: u8) -> bool {
        let placeholder = self.get(row, col);
        self.set(row, col, val);

        if
            !self.verify_row(row) ||
            !self.verify_col(col) ||
            !self.verify_box(row - (row % 3), col - (col % 3))
        {
            self.set(row, col, placeholder);
            return false;
        }

        true
    }

    pub fn get(&self, row: usize, col: usize) -> u8 {
        self.data[row][col]
    }

    pub fn set(&mut self, row: usize, col: usize, val: u8) {
        self.data[row][col] = val;
    }

    pub fn verify(&self) -> bool {
        for row in 0..9 {
            if !self.verify_row(row) {
                return false;
            }
        }
        for col in 0..9 {
            if !self.verify_col(col) {
                return false;
            }
        }
        for box_index in 0..9 {
            if !self.verify_box_by_index(box_index) {
                return false;
            }
        }
        true
    }

    fn verify_row(&self, row: usize) -> bool {
        let mut seen = HashSet::new();
        for col in 0..9 {
            let value = self.get(row, col);
            if value != 0 && seen.contains(&value) {
                return false;
            }
            seen.insert(value);
        }
        true
    }

    fn verify_col(&self, col: usize) -> bool {
        let mut seen = HashSet::new();
        for row in 0..9 {
            let value = self.get(row, col);
            if value != 0 && seen.contains(&value) {
                return false;
            }
            seen.insert(value);
        }
        true
    }

    fn verify_box_by_index(&self, box_index: usize) -> bool {
        self.verify_box(box_index / (3 as usize), box_index % 3)
    }

    // Verifies that the 3x3 box that the cell is a member of is valid.
    fn verify_box(&self, row: usize, col: usize) -> bool {
        let mut seen = HashSet::new();
        for i in 0..3 {
            for j in 0..3 {
                let value = self.get(row - (row % 3) + i, col - (col % 3) + j);
                if value != 0 && seen.contains(&value) {
                    return false;
                }
                seen.insert(value);
            }
        }
        return true;
    }
}

#[cfg(test)]
mod tests {
    use super::Sudoku;
    #[test]
    fn test_new_with_values() {
        let values = [[1, 2, 3, 4, 5, 6, 7, 8, 9]; 9];
        let sudoku = Sudoku::new_with_values(values);
        for row in 0..9 {
            for col in 0..9 {
                assert_eq!(sudoku.get(row, col), values[row][col]);
            }
        }
    }

    #[test]
    fn test_verify_then_set() {
        let mut sudoku: Sudoku = Sudoku::new();
        for i in 0..9 {
            assert!(sudoku.verify_then_set(0, i, (i + 1) as u8));
        }
    }

    #[test]
    fn test_set() {
        let mut sudoku: Sudoku = Sudoku::new();
        sudoku.set(0, 0, 1);
        assert_eq!(sudoku.get(0, 0), 1);
    }

    #[test]
    fn test_verify() {
        // Negative case
        assert!(Sudoku::new_with_values([[1, 2, 3, 4, 5, 6, 7, 8, 9]; 9]).verify());

        // Positive case
        assert!(Sudoku::new_with_values([
            [8, 3, 5, 4, 1, 6, 9, 2, 7],
            [2, 9, 6, 8, 5, 7, 4, 3, 1],
            [4, 1, 7, 2, 9, 3, 6, 5, 8],
            [5, 6, 9, 1, 3, 4, 7, 8, 2],
            [1, 2, 3, 6, 7, 8, 5, 4, 9],
            [7, 4, 8, 5, 2, 9, 1, 6, 3],
            [6, 5, 2, 7, 8, 1, 3, 9, 4],
            [9, 8, 1, 3, 4, 5, 2, 7, 6],
            [3, 7, 4, 9, 6, 2, 8, 1, 5],
        ]).verify());
    }
}
