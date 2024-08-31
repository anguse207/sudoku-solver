pub trait Cell {
    // set the nth bit to 1
    fn set_pos(&mut self, pos: u16);
    // get the bit with the nth bit set to 1
    fn get_val(&self) -> u16;
}

impl Cell for u16 {
    fn set_pos(&mut self, pos: u16) {
        *self = 0;
        *self |= 1 << pos;
    }

    fn get_val(&self) -> u16 {
        for i in 0..=15 {
            if self & (1 << i) != 0 {
                return i;
            }
        }

        panic!("No cell set");
    }
}

// TODO: Seperate the tests into seperate fns()
mod tests {
    use crate::cell_traits::Cell;

    #[test]
    fn u16_bit_ops() {
        let mut a: u16 = 0;
        a.set_pos(0);
        assert_eq!(a.get_val(), 0);

        let mut b: u16 = 0;
        b.set_pos(15);
        assert_eq!(b.get_val(), 15);

        let mut c: u16 = 0;
        c.set_pos(2);
        c.set_pos(5);
        assert_ne!(c.get_val(), 2);
        assert_eq!(c.get_val(), 5);
    }
}
