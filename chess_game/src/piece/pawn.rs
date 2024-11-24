use crate::piece::ChessPiece;
use crate::Color;

/// Represents a pawn chess piece
pub struct Pawn {
    color: Color,
}

impl ChessPiece for Pawn {
    fn get_color(&self) -> Color {
        self.color
    }

    fn get_utf_char(&self) -> char {
        match self.color {
            Color::White => '♙',
            Color::Black => '♟',
        }
    }

    fn get_fen_char(&self) -> char {
        match self.color {
            Color::White => 'P',
            Color::Black => 'p',
        }
    }
}

impl Pawn {
    pub fn new(color: Color) -> Self {
        Self { color }
    }
}
