use crate::Board;
use crate::pieces::better_piece;

pub fn start_new_chess_game() {
    let mut board = Board::new(8, 8);

    let some_pawn = better_piece::Piece {
        piece: better_piece::ChessPiece::Pawn { _has_moved: false },
        color: better_piece::PieceColor::White,
        // current_position: &board.spaces.get_node(String::from("a1")).unwrap(),
    };

    &board.spaces.get_node_mut(String::from("a1")).unwrap().set_piece(&some_pawn);
    println!("{}", board);
}
