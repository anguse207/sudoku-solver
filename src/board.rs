#![allow(unused)]

use crate::cell_traits::Cell;

pub const BOARD_SIZE: usize = 9;

pub struct Board([[u16; BOARD_SIZE]; BOARD_SIZE]);

impl Board {
    pub fn new(input: [[u16; 9]; 9]) -> Self {
        let mut board = Board([[0; BOARD_SIZE]; BOARD_SIZE]);

        for row in 0..BOARD_SIZE {
            for column in 0..BOARD_SIZE {
                board.set_cell(row, column, input[row][column]);
            }
        }

        board
    }

    pub fn get_cell(&self, row: usize, col: usize) -> u16 {
        self.0[row][col].get_val()
    }

    pub fn set_cell(&mut self, row: usize, col: usize, val: u16) {
        self.0[row][col].set_pos(val);
    }

    pub fn pretty_print(&self) {
        println!("// Board //");

        print!("[\n");
        for row in self.0.iter() {
            print!("    [");
            for cell in row.iter() {
                print!(" {:?} ", cell.get_val());
            }
            print!("],");
            println!();
        }
        println!("];");

        println!("// End //");
    }
}

mod tests {
    use crate::{
        board::{Board, BOARD_SIZE},
        cell_traits::Cell,
    };

    #[test]
    fn new_board() {
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

        let board = Board::new(input);

        board.pretty_print();

        for row in 0..BOARD_SIZE {
            for column in 0..BOARD_SIZE {
                assert_eq!(board.get_cell(row, column), input[row][column]);
            }
        }
    }
}


