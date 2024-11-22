use crate::piece::ChessPiece;
use crate::Color;

pub struct King {
    color: Color,
}

impl ChessPiece for King {
    fn get_color(&self) -> Color {
        self.color
    }

    fn get_as_char(&self) -> char {
        match self.color {
            Color::White => '♔',
            Color::Black => '♚',
        }
    }
}

impl King {
    pub fn new(color: Color) -> Self {
        Self { color }
    }
}
