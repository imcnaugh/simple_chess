use std::any::Any;
use crate::piece::ChessPiece;
use crate::Color;

/// represents a rook chess piece
pub struct Rook {
    color: Color,
}

impl ChessPiece for Rook {
    fn get_color(&self) -> Color {
        self.color
    }

    fn get_utf_char(&self) -> char {
        match self.color {
            Color::White => '♖',
            Color::Black => '♜',
        }
    }

    fn get_fen_char(&self) -> char {
        match self.color {
            Color::White => 'R',
            Color::Black => 'r',
        }
    }
}

impl Rook {
    pub fn new(color: Color) -> Self {
        Self { color }
    }
}
