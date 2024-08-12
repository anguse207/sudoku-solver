use crate::Square;

#[test]
fn bit_operations() {
    let mut s = Square::new();

    s.set_pos(0);
    s.set_pos(1);
    s.set_pos(2);
    s.set_pos(3);
    s.set_pos(4);
    s.set_pos(5);
    s.set_pos(6);
    s.set_pos(7);
    s.set_pos(8);

    assert_eq!(s.valid_pos_list(), [0,1,2,3,4,5,6,7,8]);

    s.unset_pos(0);
    s.unset_pos(1);
    s.unset_pos(2);
    s.unset_pos(3);
    s.unset_pos(4);
    s.unset_pos(5);
    s.unset_pos(6);
    s.unset_pos(7);
    s.unset_pos(8);

    assert_eq!(s.valid_pos_list(), []);
}
