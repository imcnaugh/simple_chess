use crate::chess_move::ChessMoveType;
use crate::Color;
use game_board::Board;
use std::thread::current;

mod bishop;
mod king;
mod knight;
mod pawn;
mod queen;
mod rook;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum PieceType {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

/// ChessPiece represents a single chess piece on the board.
///
/// Each ChessPiece has a specific type (Pawn, Rook, Knight, Bishop, Queen, King)
/// and a color (White or Black).
pub struct ChessPiece {
    piece_type: PieceType,
    color: Color,
}

impl ChessPiece {
    /// Creates a new `ChessPiece` with the specified type and color.
    ///
    /// # Arguments
    ///
    /// * `piece_type` - The type of the chess piece (e.g., Pawn, Rook, Knight, Bishop, Queen, King).
    /// * `color` - The color of the chess piece (e.g., White or Black).
    ///
    /// # Examples
    ///
    /// ```
    /// use chess_game::piece::{ChessPiece, PieceType};
    /// use chess_game::Color;
    ///
    /// let white_king = ChessPiece::new(PieceType::King, Color::White);
    /// let black_pawn = ChessPiece::new(PieceType::Pawn, Color::Black);
    /// ```
    pub fn new(piece_type: PieceType, color: Color) -> Self {
        Self { piece_type, color }
    }

    /// Returns the color of the chess piece.
    ///
    /// # Examples
    ///
    /// ```
    /// use chess_game::piece::{ChessPiece, PieceType};
    /// use chess_game::Color;
    ///
    /// let white_pawn = ChessPiece::new(PieceType::Pawn, Color::White);
    /// assert_eq!(white_pawn.get_color(), Color::White);
    ///
    /// let black_queen = ChessPiece::new(PieceType::Queen, Color::Black);
    /// assert_eq!(black_queen.get_color(), Color::Black);
    /// ```
    pub fn get_color(&self) -> Color {
        self.color
    }

    /// Returns the UTF-8 string representation of the chess piece based on its type and color.
    ///
    /// # Examples
    ///
    /// ```
    /// use chess_game::piece::{ChessPiece, PieceType};
    /// use chess_game::Color;
    ///
    /// let white_king = ChessPiece::new(PieceType::King, Color::White);
    /// assert_eq!(white_king.as_utf_str(), "♔");
    ///
    /// let black_pawn = ChessPiece::new(PieceType::Pawn, Color::Black);
    /// assert_eq!(black_pawn.as_utf_str(), "♟");
    /// ```
    pub fn as_utf_str(&self) -> &str {
        match self.piece_type {
            PieceType::King => king::as_utf_str(self.color),
            PieceType::Queen => queen::as_utf_str(self.color),
            PieceType::Rook => rook::as_utf_str(self.color),
            PieceType::Bishop => bishop::as_utf_str(self.color),
            PieceType::Knight => knight::as_utf_str(self.color),
            PieceType::Pawn => pawn::as_utf_str(self.color),
        }
    }

    /// Returns the FEN (Forsyth-Edwards Notation) character representation of the chess piece
    /// based on its type and color.
    ///
    /// # Examples
    ///
    /// ```
    /// use chess_game::piece::{ChessPiece, PieceType};
    /// use chess_game::Color;
    ///
    /// let white_king = ChessPiece::new(PieceType::King, Color::White);
    /// assert_eq!(white_king.as_fen_char(), 'K');
    ///
    /// let black_pawn = ChessPiece::new(PieceType::Pawn, Color::Black);
    /// assert_eq!(black_pawn.as_fen_char(), 'p');
    /// ```
    pub fn as_fen_char(&self) -> char {
        match self.piece_type {
            PieceType::King => king::as_fen_char(self.color),
            PieceType::Queen => queen::as_fen_char(self.color),
            PieceType::Rook => rook::as_fen_char(self.color),
            PieceType::Bishop => bishop::as_fen_char(self.color),
            PieceType::Knight => knight::as_fen_char(self.color),
            PieceType::Pawn => pawn::as_fen_char(self.color),
        }
    }

    pub fn possible_moves(
        &self,
        position: (usize, usize),
        board: Board<ChessPiece>,
    ) -> Vec<ChessMoveType> {
        match self.piece_type {
            PieceType::King => king::possible_moves(self.color, position, board),
            PieceType::Queen => queen::possible_moves(self.color, position, board),
            PieceType::Rook => rook::possible_moves(self.color, position, board),
            PieceType::Bishop => bishop::possible_moves(self.color, position, board),
            PieceType::Knight => knight::possible_moves(self.color, position, board),
            PieceType::Pawn => pawn::possible_moves(self.color, position, board),
        }
    }
}
