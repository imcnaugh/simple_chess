use crate::chess_piece::ChessPiece;
use crate::color::Color;

/// Represents a pawn chess piece
pub struct Pawn {
    color: Color,
}

impl ChessPiece for Pawn {
    fn get_color(&self) -> Color {
        self.color
    }

    fn get_as_char(&self) -> char {
        match self.color {
            Color::White => '♙',
            Color::Black => '♟',
        }
    }
}

impl Pawn {
    /// This module defines the `Pawn` struct, which implements the 
    /// `ChessPiece` trait and represents a pawn chess piece.

    /// The `Pawn` struct represents a pawn in a chess game. It contains
    /// a field for the color of the pawn, which can be either white or black.
    pub fn new(color: Color) -> Self {
        Self {
            color
        }
    }
}