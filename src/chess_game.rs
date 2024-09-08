use crate::Board;
use crate::board_square::BoardSquare;
use crate::pieces::better_piece;

pub fn start_new_chess_game() {
    let mut board = Board::new(8, 8);

    let mut some_pawn = better_piece::Piece {
        piece: better_piece::ChessPiece::Pawn { _has_moved: false },
        color: better_piece::PieceColor::White,
        board_square_id: None,
    };
    let mut some_queen = better_piece::Piece {
        piece: better_piece::ChessPiece::Queen,
        color: better_piece::PieceColor::Black,
        board_square_id: None,
    };

    let mut a1_space = board.spaces.get_node_mut(String::from("a1")).unwrap();
    set_piece_on_space(&mut some_pawn, &mut a1_space);

    let mut a2_space = board.spaces.get_node_mut(String::from("a2")).unwrap();
    set_piece_on_space(&mut some_queen, &mut a2_space);


    board.spaces.get_node_mut(String::from("a1")).unwrap().clear_piece();
    println!("{}", board);
}

fn set_piece_on_space<'a>(piece: &'a mut better_piece::Piece, space: &mut BoardSquare<'a>) {
    piece.board_square_id = Some(space.name.clone());
    space.set_piece(piece);
}
