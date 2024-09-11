use crate::Color;
use std::fmt;

pub enum PieceType {
    Pawn { has_moved: bool },
    Rook { has_moved: bool },
    Knight,
    Bishop,
    Queen,
    King { has_moved: bool },
}

pub struct ChessPiece {
    color: Color,
    piece_type: PieceType,
}

impl ChessPiece {
    pub fn new(color: Color, piece_type: PieceType) -> ChessPiece {
        ChessPiece { color, piece_type }
    }
}

impl PieceType {
    fn get_as_utf_char(&self) -> char {
        match self {
            PieceType::Pawn { has_moved: _ } => '♙',
            PieceType::Rook { has_moved: _ } => '♖',
            PieceType::Knight => '♘',
            PieceType::Bishop => '♗',
            PieceType::Queen => '♕',
            PieceType::King { has_moved: _ } => '♔',
        }
    }
}

impl fmt::Display for ChessPiece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut display_char = self.piece_type.get_as_utf_char();
        if self.color == Color::Black {
            display_char = char::from_u32(display_char as u32 + 6).unwrap();
        }

        write!(f, "{}", display_char,)
    }
}
