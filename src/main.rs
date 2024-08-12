struct board {
    board: [[square; 8]; 8]
}

type bitmask = u16;

struct square { 
    mask: bitmask
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

    fn is_flipped(&self, bit: bitmask) -> bool {
        self.mask & bit != 0
    }
}

fn main() {
    let mut s = square::new();

    println!("{}", s.is_flipped(1));
    s.flip(1);
    println!("{}", s.is_flipped(1));
    println!("{}", s.is_flipped(0));
}
