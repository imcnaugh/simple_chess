use crate::Board;

pub fn start_new_chess_game() {
    let board = Board::new(8, 8);
    println!("{}", board);
}
