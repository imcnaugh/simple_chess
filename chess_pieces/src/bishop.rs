use crate::{ChessPiece, Color};

pub struct Bishop {
    color: Color,
}

impl ChessPiece for Bishop {
    fn get_color(&self) -> Color {
        self.color
    }

    fn get_as_char(&self) -> char {
        match self.color {
            Color::White => '♗',
            Color::Black => '♝',
        }
    }
}

impl Bishop {
    pub fn new(color: Color) -> Self {
        Self { color }
    }
}
