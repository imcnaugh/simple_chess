use chess::Color;
use chess::chess_piece::{ChessPiece, PieceType};

fn main() {
    println!("Hello, world!");

    let white_pieces = create_piece_set(Color::White);
    let black_pieces = create_piece_set(Color::Black);

    let mut empty_board = [[None; 8]; 8];

    let mut square_a1 = empty_board[0][0];

    square_a1 = Some(&white_pieces[0]);

    for piece in white_pieces {
        println!("{}", piece);
    }

    for piece in black_pieces {
        println!("{}", piece);
    }
}

fn create_piece_set(color: Color) -> Vec<ChessPiece> {
    let mut pieces = Vec::new();
    for _ in 0..8 {
        pieces.push(ChessPiece::new(color, PieceType::Pawn));
    }
    for _ in 0..2 {
        pieces.push(ChessPiece::new(color, PieceType::Rook));
        pieces.push(ChessPiece::new(color, PieceType::Knight));
        pieces.push(ChessPiece::new(color, PieceType::Bishop));
    }
    pieces.push(ChessPiece::new(color, PieceType::Queen));
    pieces.push(ChessPiece::new(color, PieceType::King));
    pieces
}

struct Square {
    piece: Option<ChessPiece>,
}
