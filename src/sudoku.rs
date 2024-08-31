#![allow(unused)]

use crate::{board::{Board, BOARD_SIZE}, cell_traits::Cell};

struct Sudoku(Board);

impl Sudoku {
    fn new(board: Board) -> Self {
        Sudoku(board)
    }

    fn is_safe(&self, row: u16, col: u16, num: u16) -> bool {
        for i in 0..BOARD_SIZE {
            if self.0.get_cell(row as usize, i) == num {
                return false;
            }
        }

        for i in 0..BOARD_SIZE {
            if self.0.get_cell(i, col as usize) == num {
                return false;
            }
        }

        let start_row = row - row % 3;
        let start_col = col - col % 3;

        for i in 0..3 {
            for j in 0..3 {
                if self.0.get_cell(i + start_row as usize, j + start_col as usize) == num {
                    return false;
                }
            }
        }

        true
    }

    fn find_empty(&self) -> Option<(u16, u16)> {
        for row_i in 0..BOARD_SIZE {
            for col_i in 0..BOARD_SIZE {
                match self.0.get_cell(row_i as usize, col_i as usize) {
                    0 => return Some((row_i as u16, col_i as u16)),
                    _ => continue,
                }
            }
        }

        println!("No empty squares found");
        None
    }

    pub fn solve(&mut self) -> bool {
        if let Some((row, col)) = self.find_empty() {
            for i in 1..=9 {
                if self.is_safe(row, col, i) {
                    self.0.set_cell(row as usize, col as usize, i);

                    if self.solve() {
                        return true;
                    }

                    self.0.set_cell(row as usize, col as usize, 0);
                }
            }

            return false;
        }

        self.0.pretty_print();
        true
    }
}

mod tests {
    use crate::{
        board::{Board, BOARD_SIZE},
        cell_traits::Cell,
        sudoku::Sudoku,
    };

    #[test]
    fn bench() {
        let now = std::time::Instant::now();
        
        let mut game: Sudoku = test_setup();

        game.solve();

        let elapsed = now.elapsed();
        println!("Elapsed: {:?}", elapsed);
    }

    #[test]
    fn solve() {
        let mut game: Sudoku = test_setup();
        
        assert_eq!(game.solve(), true);
    }

    #[test]
    fn is_safe() {
        let game: Sudoku = test_setup();
        // Test the row comparison, col comparison, and square comparison
        assert_eq!(game.is_safe(0, 0, 5), false);
        assert_eq!(game.is_safe(0, 2, 2), true);

        assert_eq!(game.is_safe(8, 0, 5), false);
        assert_eq!(game.is_safe(8, 8, 9), false);

        assert_eq!(game.is_safe(7, 7, 2), false);
        assert_eq!(game.is_safe(1, 1, 8), false);
    }

    #[test]
    fn find_empty() {
        // Create a filled grid
        let mut grid: [[u16; 9]; 9] = [[0; 9]; 9];
        for row in 0..BOARD_SIZE {
            for col in 0..BOARD_SIZE {
                grid[row][col] = 1;
            }
        }

        let game = Sudoku::new(Board::new(grid));
        assert_eq!(game.find_empty(), None);

        let game: Sudoku = test_setup();
        assert_eq!(game.find_empty(), Some((0, 2)));
    }

    fn test_setup() -> Sudoku {
        let input: [[u16; 9]; 9] = [
            [5, 3, 0, 0, 7, 0, 0, 0, 0],
            [6, 0, 0, 1, 9, 5, 0, 0, 0],
            [0, 9, 8, 0, 0, 0, 0, 6, 0],
            [8, 0, 0, 0, 6, 0, 0, 0, 3],
            [4, 0, 0, 8, 0, 3, 0, 0, 1],
            [7, 0, 0, 0, 2, 0, 0, 0, 6],
            [0, 6, 0, 0, 0, 0, 2, 8, 0],
            [0, 0, 0, 4, 1, 9, 0, 0, 5],
            [0, 0, 0, 0, 8, 0, 0, 7, 9],
        ];

        Sudoku::new(Board::new(input))
    }
}
