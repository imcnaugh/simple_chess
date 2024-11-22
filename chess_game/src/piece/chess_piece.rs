use crate::color::Color;

/// A trait representing a generic chess piece.
///
/// This trait defines the necessary functions that any chess piece should implement.
///
/// # Required Methods
///
/// - `get_color`: Returns the color of the chess piece.
/// - `get_position`: Returns the position of the chess piece on the board.
/// - `get_as_char`: Returns the chess piece as a character representation.
///
/// # Examples
///
/// ```
/// // Assuming there is a struct `Pawn` that implements `ChessPiece`
/// use chess_game::Color;
/// use chess_game::piece::{Pawn, ChessPiece};
/// let pawn = Pawn::new(Color::White);
/// assert_eq!(pawn.get_color(), Color::White);
/// assert_eq!(pawn.get_as_char(), '♙');
/// ```
pub trait ChessPiece {
    fn get_color(&self) -> Color;

    fn get_as_char(&self) -> char;
}
