use chess_board::Piece;
use std::any::Any;

#[derive(Debug, PartialEq, Eq)]
pub struct CheckersPiece {}

impl Piece for CheckersPiece {
    fn get_char_representation(&self) -> char {
        'o'
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl CheckersPiece {
    pub fn new() -> Self {
        Self {}
    }
}

pub enum ChessPiece {
    Pawn,
    Rook,
    Bishop,
}

impl Piece for ChessPiece {
    fn get_char_representation(&self) -> char {
        match self {
            ChessPiece::Pawn => 'p',
            ChessPiece::Rook => 'r',
            ChessPiece::Bishop => 'b',
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
