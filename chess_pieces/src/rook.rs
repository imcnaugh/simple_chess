use crate::{ChessPiece, Color};

/// represents a rook chess piece
pub struct Rook {
    color: Color,
}

impl ChessPiece for Rook {
    fn get_color(&self) -> Color {
        self.color
    }

    fn get_as_char(&self) -> char {
        match self.color {
            Color::White => '♖',
            Color::Black => '♜',
        }
    }
}

impl Rook {
    pub fn new(color: Color) -> Self {
        Self { color }
    }
}
