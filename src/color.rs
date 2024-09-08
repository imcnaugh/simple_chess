/// Color module
///
/// This module contains the Color enum, which represents the color of either a chess piece or a
/// square on the board.
#[derive(Clone, Copy)]
pub enum Color {
    White,
    Black,
}

impl Color {
    pub fn ascii_square_color(&self) -> &str {
        match self {
            Color::White => "\x1b[107m",
            Color::Black => "\x1b[100m",
        }
    }

    pub fn ascii_piece_color(&self) -> &str {
        match self {
            Color::White => "\x1b[97m",
            Color::Black => "\x1b[30m",
        }
    }
}