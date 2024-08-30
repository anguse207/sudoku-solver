use std::fmt::{Debug, Display};


#[cfg(test)]
mod tests;

type Row = [Bitmask; 9];

#[derive(Debug, Clone, Copy)]
pub struct Board {
    board: [Row; 9],
}

impl Board {
    // TODO: make this generic
    pub fn new(grid: [[u16; 9]; 9]) -> Board {
        let mut board: [[Bitmask; 9]; 9] = [[0; 9]; 9];

        for (row_i, row) in grid.iter().enumerate() {
            for (col_i, cell) in row.iter().enumerate() {
                let square = Square::new(*cell);
                board[row_i][col_i] = square.mask;              
            }
        }

        Board { board }
    }

    pub fn set_square(&mut self, row: u16, col: u16, val: u16) {
        self.board[row as usize][col as usize] = 0;
        self.board[row as usize][col as usize] = set_mask(val);
    }

    pub fn get_square(&self, row: u16, col: u16) -> u16 {
        self.board[row as usize][col as usize]
    }

    fn is_safe(&self, num: u16, row: u16, col: u16) -> bool {
        for i in 0..8 {
            if self.get_square(row, i) == get_mask(num) {
                return false;
            }
        }

        for i in 0..8 {
            if self.get_square(i, col) == get_mask(num) {
                return false;
            }
        }

        let start_row = row - row % 3;
        let start_col = col - col % 3;

        for i in 0..3 {
            for j in 0..3 {
                if self.get_square(i + start_row, j + start_col) == get_mask(num) {
                    return false;
                }
            }
        }
        
        true
    }

    pub fn solve(&mut self) -> bool {
        if let Some((row, col)) = self.find_empty() {
            self.pretty_print();
            for i in 1..=9 {
                if self.is_safe(i, row, col) {
                    self.set_square(row, col, i);

                    if self.solve() {
                        return true;
                    }

                    self.set_square(row, col, 0);
                }
            }

            return false;
        }

        false
    }

    fn find_empty(&self) -> Option<(u16, u16)> {
        for row_i in 0..=8 {
            for col_i in 0..=8 {
                match self.get_square(row_i, col_i) {
                    0 => return Some((row_i as u16, col_i as u16)),
                    _ => continue,
                }
            }
        }

        println!("No empty squares found");
        None
    }

    pub fn pretty_print(&self) {
        println!("Board:");
        for row in self.board.iter() {
            for square in row.iter() {
                print!("{:?} ", square);
            }
            println!();
        }
    }
}

fn set_mask(pos: u16) -> Bitmask {
    let mut mask: Bitmask = 0x0000;
    mask ^ (1 << pos)
}

fn get_mask(mask: Bitmask) -> u16 {
    1 << mask
}

type Bitmask = u16;

#[derive(Copy, Clone)]
pub struct Square {
    mask: Bitmask,
}

// impl debug for square
impl Debug for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let vals = self.valid_pos_list();
        match vals.len() {
            0 => write!(f, "0"),
            _ => write!(f, "{}", vals[0]),
        }
    }
}

impl Display for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self.valid_pos_list())
    }
}

#[derive(Debug)]
enum SquareError {
    OOR(String),
}

impl Square {
    pub fn new(val: u16) -> Square {
        let mut s = Square { mask: 0 };
        s.set_pos(val);
        s
    }

    fn valid_pos_list(&self) -> Vec<u16> {
        let mut valid = Vec::new();

        for i in 1..=9 {
            if self.is_pos_set(i) {
                valid.push(i);
            }
        }

        valid
    }

    fn set_pos(&mut self, pos: u16) {
        self.mask = 0;
        self.mask |= 1 << (pos);
    }

    fn unset_pos(&mut self, pos: u16) {
        self.mask = self.mask & !(1 << pos);
    }

    fn is_pos_set(&self, pos: u16) -> bool {
        (self.mask & (1 << pos)) != 0
    }

    pub fn is_solved(&self) -> bool {
        self.mask != 0
    }

    pub fn new_empty() -> Square {
        Square { mask: 0 }
    }
}

