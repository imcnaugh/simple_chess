use crate::{ChessPiece, Color};

pub struct Queen {
    color: Color,
}

impl ChessPiece for Queen {
    fn get_color(&self) -> Color {
        self.color
    }

    fn get_as_char(&self) -> char {
        match self.color {
            Color::White => '♕',
            Color::Black => '♛',
        }
    }
}

impl Queen {
    pub fn new(color: Color) -> Self {
        Self { color }
    }
}
