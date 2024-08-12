struct Board {
    board: [[Square; 8]; 8],
}

type Bitmask = u16;

struct Square {
    mask: Bitmask,
}

#[derive(Debug)]
enum SquareError {
    OOR(String),
}

impl Square {
    pub fn new() -> Square {
        Square { mask: 0 }
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

#[cfg(test)]
mod tests;