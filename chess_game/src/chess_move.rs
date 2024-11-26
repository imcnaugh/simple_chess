use crate::piece::ChessPiece;

pub enum ChessMoveType {
    Move {
        original_position: (usize, usize),
        new_position: (usize, usize),
        piece: ChessPiece,
        taken_piece: Option<ChessPiece>,
        promotion: Option<ChessPiece>,
    },
}
