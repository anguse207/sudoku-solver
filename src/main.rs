use std::{error::Error, string};

struct board {
    board: [[square; 8]; 8]
}

type bitmask = u16;

struct square { 
    mask: bitmask
}

enum SquareError {
    Range(String),
}

impl square {
    pub fn new() -> square {
        square { 
            mask: 0 
        }
    }

    fn flip(&mut self, bit: bitmask) {
        self.mask ^= bit;
    }

    fn is_flipped(&self, bit: bitmask) -> Result<bool, SquareError> {
        if bit > 9 {
            return Err(SquareError::Range("bitmask out of range".to_string()));
        }

        Ok(self.mask & bit != 0)
    }
}

fn main() {
    let mut s = square::new();

    s.flip(1);
}
