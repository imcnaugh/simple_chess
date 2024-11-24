use crate::piece::ChessPiece;
use crate::Color;

pub struct Knight {
    color: Color,
}

impl ChessPiece for Knight {
    fn get_color(&self) -> Color {
        self.color
    }

    fn get_utf_char(&self) -> char {
        match self.color {
            Color::White => '♘',
            Color::Black => '♞',
        }
    }

    fn get_fen_char(&self) -> char {
        match self.color {
            Color::White => 'N',
            Color::Black => 'n',
        }
    }
}

impl Knight {
    pub fn new(color: Color) -> Self {
        Self { color }
    }
}
