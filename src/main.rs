use chess::Color;
use chess::chess_piece::{ChessPiece, PieceType};

fn main() {
    println!("Hello, world!");

    let white_pawn = ChessPiece::new(Color::White, PieceType::Pawn);
    let black_queen = ChessPiece::new(Color::Black, PieceType::Queen);

    println!("{}", white_pawn);
    println!("{}", black_queen);
}
