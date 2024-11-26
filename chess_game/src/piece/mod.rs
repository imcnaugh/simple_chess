mod bishop;
mod king;
mod knight;
mod pawn;
mod queen;
mod rook;

use crate::color::Color;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum PieceType {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

pub struct ChessPiece {
    piece_type: PieceType,
    color: Color,
}

impl ChessPiece {
    pub fn new(piece_type: PieceType, color: Color) -> Self {
        Self { piece_type, color }
    }

    pub fn get_color(&self) -> Color {
        self.color
    }

    pub fn as_utf_str(&self) -> &str {
        match self.piece_type {
            PieceType::King => king::as_utf_str(self.color),
            PieceType::Queen => queen::as_utf_str(self.color),
            PieceType::Rook => rook::as_utf_str(self.color),
            PieceType::Bishop => bishop::as_utf_str(self.color),
            PieceType::Knight => knight::as_utf_str(self.color),
            PieceType::Pawn => pawn::as_utf_str(self.color),
        }
    }

    pub fn as_fen_char(&self) -> char {
        match self.piece_type {
            PieceType::King => king::as_fen_char(self.color),
            PieceType::Queen => queen::as_fen_char(self.color),
            PieceType::Rook => rook::as_fen_char(self.color),
            PieceType::Bishop => bishop::as_fen_char(self.color),
            PieceType::Knight => knight::as_fen_char(self.color),
            PieceType::Pawn => pawn::as_fen_char(self.color),
        }
    }
}
