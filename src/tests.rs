use crate::*;

// #[test]
// fn bit_operations() {
//     let mut s = Square::new(0);

//     s.set_pos(1);
//     s.set_pos(2);
//     s.set_pos(3);
//     s.set_pos(4);
//     s.set_pos(5);
//     s.set_pos(6);
//     s.set_pos(7);
//     s.set_pos(8);
//     s.set_pos(9);

//     assert_eq!(s.valid_pos_list(), [1,2,3,4,5,6,7,8,9]);

//     s.unset_pos(1);
//     s.unset_pos(2);
//     s.unset_pos(3);
//     s.unset_pos(4);
//     s.unset_pos(5);
//     s.unset_pos(6);
//     s.unset_pos(7);
//     s.unset_pos(8);
//     s.unset_pos(9);

//     assert_eq!(s.valid_pos_list(), []);
// }


#[test]
fn bit_ops() {
    let mut cell: Bitmask = 0;

    let cell = set_mask(2);
    let val = get_mask(cell);
    println!("Cell: {:016b}", cell);
    println!("Val: {}", val);
}

// #[test]
fn solve() {
    let grid: [[u16; 9]; 9] = [
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
    println!("Input {:?}", grid);

    let mut board = Board::new(grid);

    let result = board.solve();
    println!("Result: {:?}", result);
    board.pretty_print();
    

}