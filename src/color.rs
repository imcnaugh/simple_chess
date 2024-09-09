/// Color module
///
/// This module contains the Color enum, which represents the color of either a chess piece or a
/// square on the board.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Color {
    White,
    Black,
}
