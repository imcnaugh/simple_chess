use crate::{ChessPiece, Color};

pub struct Knight {
    color: Color,
}

impl ChessPiece for Knight {
    fn get_color(&self) -> Color {
        self.color
    }

    fn get_as_char(&self) -> char {
        match self.color {
            Color::White => '♘',
            Color::Black => '♞',
        }
    }
}

impl Knight {
    pub fn new(color: Color) -> Self {
        Self { color }
    }
}
