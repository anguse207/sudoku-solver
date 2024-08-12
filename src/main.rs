use std::{error::Error, string};

struct board {
    board: [[square; 8]; 8],
}

type bitmask = u16;

struct square {
    mask: bitmask,
}

#[derive(Debug)]
enum SquareError {
    OOR(String),
}

impl square {
    pub fn new() -> square {
        square { mask: 0 }
    }

    fn valid_pos_list(&self) -> Vec<u16> {
        let mut valid = Vec::new();

        for i in 0..=8 {
            if self.is_pos_set(i) {
                valid.push(i);
            }
        }

        valid
    }

    fn set_pos(&mut self, pos: u16) {
        self.mask |= 1 << (pos);
    }

    fn unset_pos(&mut self, pos: u16) {
        self.mask = self.mask & !(1 << pos);
    }

    fn is_pos_set(&self, pos: u16) -> bool {
        (self.mask & (1 << pos)) != 0
    }
}

fn main() {
    let mut s = square::new();

    s.set_pos(0);

    s.set_pos(4);
    s.unset_pos(4);

    s.set_pos(8);

    println!("{:?}", s.is_pos_set(1));

    println!("{:?}", s.valid_pos_list());
}
