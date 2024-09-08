use crate::Color;
use std::fmt;

pub enum PieceType {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

pub struct ChessPiece {
    color: Color,
    piece_type: PieceType,
}

impl ChessPiece {
    pub fn new(color: Color, piece_type: PieceType) -> ChessPiece {
        ChessPiece { color, piece_type }
    }

    pub fn get_piece_type(&self) -> &PieceType {
        &self.piece_type
    }
}

impl PieceType {
    fn ascii_representation(&self) -> &str {
        match self {
            PieceType::Pawn => "P",
            PieceType::Rook => "R",
            PieceType::Knight => "N",
            PieceType::Bishop => "B",
            PieceType::Queen => "Q",
            PieceType::King => "K",
        }
    }
}

impl fmt::Display for ChessPiece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}{}{}",
            self.color.ascii_piece_color(),
            self.piece_type.ascii_representation(),
            "\x1b[0m"
        )
    }
}
