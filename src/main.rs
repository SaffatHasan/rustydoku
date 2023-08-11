mod display;
mod solver;
mod sudoku;
mod sudoku_error;
use crate::sudoku::Sudoku;
use eframe::egui;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(320.0, 240.0)),
        ..Default::default()
    };
    eframe::run_native("Sudoku", options, Box::new(|_cc| Box::<MyApp>::default()))
}

struct MyApp {
    board: Sudoku,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            board: Sudoku::new_with_values([
                [0, 0, 0, 0, 0, 0, 0, 0, 0],
                [2, 9, 6, 8, 5, 7, 4, 3, 1],
                [4, 1, 7, 2, 9, 3, 6, 5, 8],
                [5, 6, 9, 1, 3, 4, 7, 8, 2],
                [1, 2, 3, 6, 7, 8, 5, 4, 9],
                [7, 4, 8, 5, 2, 9, 1, 6, 3],
                [6, 5, 2, 7, 8, 1, 3, 9, 4],
                [9, 8, 1, 3, 4, 5, 2, 7, 6],
                [3, 7, 4, 9, 6, 2, 8, 1, 5],
            ]),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::Grid::new("sudoku_grid").show(ui, |ui| {
                for row in 0..9 {
                    for col in 0..9 {
                        ui.label(format!("{}", self.board.get(row, col)));
                    }
                    ui.end_row();
                }
            });

            if ui.button("Solve").clicked() {
                self.board = solver::solve_sudoku(&self.board).unwrap();
            }

            if ui.button("Reset").clicked() {
                self.board = MyApp::default().board
            }
        });
    }
}
