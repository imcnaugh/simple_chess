use crate::piece::ChessPiece;
use crate::Color;
use std::any::Any;

pub struct Queen {
    color: Color,
}

impl ChessPiece for Queen {
    fn get_color(&self) -> Color {
        self.color
    }

    fn get_utf_char(&self) -> char {
        match self.color {
            Color::White => '♕',
            Color::Black => '♛',
        }
    }

    fn get_fen_char(&self) -> char {
        match self.color {
            Color::White => 'Q',
            Color::Black => 'q',
        }
    }
}

impl Queen {
    pub fn new(color: Color) -> Self {
        Self { color }
    }
}
