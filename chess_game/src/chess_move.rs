use crate::piece::ChessPiece;

#[derive(Debug, PartialEq)]
pub enum ChessMoveType {
    Move {
        original_position: (usize, usize),
        new_position: (usize, usize),
        piece: ChessPiece,
        taken_piece: Option<ChessPiece>,
        promotion: Option<ChessPiece>,
    },
    EnPassant {
        original_position: (usize, usize),
        new_position: (usize, usize),
        piece: ChessPiece,
        taken_piece: ChessPiece,
        taken_piece_position: (usize, usize),
        promotion: Option<ChessPiece>,
    },
}
