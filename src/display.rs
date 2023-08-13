use crate::Sudoku;

impl std::fmt::Display for Sudoku {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut output = String::new();
        for row in 0..9 {
            if row % 3 == 0 {
                output.push_str("+---+---+---+\n");
            }
            output.push('|');

            for col in 0..9 {
                output.push_str(format!("{}", self.get(row, col)).as_str());
                if col == 2 || col == 5 {
                    output.push('|');
                }
            }
            output.push_str("|\n");
        }
        output.push_str("+---+---+---+");
        write!(f, "{}", output)
    }
}

#[cfg(test)]
mod testsalso {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_display() {
        assert_eq!(
            format!(
                "{}",
                Sudoku::new([[1, 2, 3, 4, 5, 6, 7, 8, 9]; 9])
            ),
            r#"+---+---+---+
|123|456|789|
|123|456|789|
|123|456|789|
+---+---+---+
|123|456|789|
|123|456|789|
|123|456|789|
+---+---+---+
|123|456|789|
|123|456|789|
|123|456|789|
+---+---+---+"#
        );
        let mut doku = Sudoku::default();
        for row in 0..9 {
            for col in 0..9 {
                doku = doku.set(row, col, (row / 3 % 3 * 3 + (col / 3)) as u8);
            }
        }
        assert_eq!(
            format!("{}", doku),
            r#"+---+---+---+
|000|111|222|
|000|111|222|
|000|111|222|
+---+---+---+
|333|444|555|
|333|444|555|
|333|444|555|
+---+---+---+
|666|777|888|
|666|777|888|
|666|777|888|
+---+---+---+"#
        );
    }
}
