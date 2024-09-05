use chess::Board;
use chess::pieces::{Bishop, King, Knight, Pawn, PieceColor, Queen, Rook};

fn main() {
    let mut new_board = Board::new(8, 8);

    new_board.spaces.get_node_mut(String::from("a1"))
        .unwrap()
        .set_piece(Box::new(Rook::new(PieceColor::White)));
    new_board.spaces.get_node_mut(String::from("b1"))
        .unwrap()
        .set_piece(Box::new(Knight::new(PieceColor::White)));
    new_board.spaces.get_node_mut(String::from("c1"))
        .unwrap()
        .set_piece(Box::new(Bishop::new(PieceColor::White)));
    new_board.spaces.get_node_mut(String::from("d1"))
        .unwrap()
        .set_piece(Box::new(Queen::new(PieceColor::White)));
    new_board.spaces.get_node_mut(String::from("e1"))
        .unwrap()
        .set_piece(Box::new(King::new(PieceColor::White)));
    new_board.spaces.get_node_mut(String::from("f1"))
        .unwrap()
        .set_piece(Box::new(Bishop::new(PieceColor::White)));
    new_board.spaces.get_node_mut(String::from("g1"))
        .unwrap()
        .set_piece(Box::new(Knight::new(PieceColor::White)));
    new_board.spaces.get_node_mut(String::from("h1"))
        .unwrap()
        .set_piece(Box::new(Rook::new(PieceColor::White)));

    for col in 'a'..='h' {
        let key = format!("{}2", col);
        new_board.spaces.get_node_mut(key)
            .unwrap()
            .set_piece(Box::new(Pawn::new(PieceColor::White)));
    }

    new_board.spaces.get_node_mut(String::from("a8"))
        .unwrap()
        .set_piece(Box::new(Rook::new(PieceColor::Black)));
    new_board.spaces.get_node_mut(String::from("b8"))
        .unwrap()
        .set_piece(Box::new(Knight::new(PieceColor::Black)));
    new_board.spaces.get_node_mut(String::from("c8"))
        .unwrap()
        .set_piece(Box::new(Bishop::new(PieceColor::Black)));
    new_board.spaces.get_node_mut(String::from("d8"))
        .unwrap()
        .set_piece(Box::new(Queen::new(PieceColor::Black)));
    new_board.spaces.get_node_mut(String::from("e8"))
        .unwrap()
        .set_piece(Box::new(King::new(PieceColor::Black)));
    new_board.spaces.get_node_mut(String::from("f8"))
        .unwrap()
        .set_piece(Box::new(Bishop::new(PieceColor::Black)));
    new_board.spaces.get_node_mut(String::from("g8"))
        .unwrap()
        .set_piece(Box::new(Knight::new(PieceColor::Black)));
    new_board.spaces.get_node_mut(String::from("h8"))
        .unwrap()
        .set_piece(Box::new(Rook::new(PieceColor::Black)));

    for col in 'a'..='h' {
        let key = format!("{}7", col);
        new_board.spaces.get_node_mut(key)
            .unwrap()
            .set_piece(Box::new(Pawn::new(PieceColor::Black)));
    }

    println!("{}", new_board);
}
