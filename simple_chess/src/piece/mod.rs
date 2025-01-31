use crate::chess_move::ChessMoveType;
use crate::Color;
use game_board::Board;
use std::fmt::{Display, Formatter};

mod bishop;
mod king;
mod knight;
mod pawn;
mod queen;
mod rook;

#[derive(Debug, PartialEq, Clone, Copy, Eq)]
pub enum PieceType {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

/// ChessPiece represents a single simple_chess piece on the board.
///
/// Each ChessPiece has a specific type (Pawn, Rook, Knight, Bishop, Queen, King)
/// and a color (White or Black).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ChessPiece {
    piece_type: PieceType,
    color: Color,
}

impl ChessPiece {
    /// Creates a new `ChessPiece` with the specified type and color.
    ///
    /// # Arguments
    ///
    /// * `piece_type` - The type of the simple_chess piece (e.g., Pawn, Rook, Knight, Bishop, Queen, King).
    /// * `color` - The color of the simple_chess piece (e.g., White or Black).
    ///
    /// # Examples
    ///
    /// ```
    /// use simple_chess::piece::{ChessPiece, PieceType};
    /// use simple_chess::Color;
    ///
    /// let white_king = ChessPiece::new(PieceType::King, Color::White);
    /// let black_pawn = ChessPiece::new(PieceType::Pawn, Color::Black);
    /// ```
    pub fn new(piece_type: PieceType, color: Color) -> Self {
        Self { piece_type, color }
    }

    /// Returns the color of the simple_chess piece.
    ///
    /// # Examples
    ///
    /// ```
    /// use simple_chess::piece::{ChessPiece, PieceType};
    /// use simple_chess::Color;
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

    /// Returns the type of the simple_chess piece.
    ///
    /// # Examples
    ///
    /// ```
    /// use simple_chess::piece::{ChessPiece, PieceType};
    /// use simple_chess::Color;
    ///
    /// let white_pawn = ChessPiece::new(PieceType::Pawn, Color::White);
    /// assert_eq!(white_pawn.get_piece_type(), PieceType::Pawn);
    ///
    /// let black_queen = ChessPiece::new(PieceType::Queen, Color::Black);
    /// assert_eq!(black_queen.get_piece_type(), PieceType::Queen);
    /// ```
    pub fn get_piece_type(&self) -> PieceType {
        self.piece_type
    }

    /// Returns the UTF-8 string representation of the simple_chess piece based on its type and color.
    ///
    /// # Examples
    ///
    /// ```
    /// use simple_chess::piece::{ChessPiece, PieceType};
    /// use simple_chess::Color;
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

    /// Returns the PGN (Portable Game Notation) character representation of the simple_chess piece.
    ///
    /// PGN is a common notation for representing chess moves.
    ///
    /// # Examples
    ///
    /// ```
    /// use simple_chess::piece::{ChessPiece, PieceType};
    /// use simple_chess::Color;
    ///
    /// // PGN character for a white queen
    /// let white_queen = ChessPiece::new(PieceType::Queen, Color::White);
    /// assert_eq!(white_queen.as_pgn_char(), Some('Q'));
    ///
    /// // Pawns typically have no PGN character
    /// let black_pawn = ChessPiece::new(PieceType::Pawn, Color::Black);
    /// assert_eq!(black_pawn.as_pgn_char(), None);
    /// ```
    pub fn as_pgn_char(&self) -> Option<char> {
        match self.piece_type {
            PieceType::Pawn => pawn::as_pgn_char(),
            PieceType::Rook => rook::as_pgn_char(),
            PieceType::Knight => knight::as_pgn_char(),
            PieceType::Bishop => bishop::as_pgn_char(),
            PieceType::Queen => queen::as_pgn_char(),
            PieceType::King => king::as_pgn_char(),
        }
    }

    /// Returns a vector of possible moves for the simple_chess piece from a given position on the board.
    ///
    /// This function computes the possible moves for the simple_chess piece, based on its type, current
    /// position on the board, and the state of the board. The rules for valid moves for each
    /// type of simple_chess piece are applied.
    ///
    /// Pins are not taken into account in this function, you will need to filter the returned
    /// vector of moves based off if they leave the king in check or not.
    ///
    /// # Arguments
    ///
    /// * `position` - A tuple `(usize, usize)` representing the current position of the simple_chess piece
    ///                on the board (row, column).
    /// * `board` - A reference to the `Board<ChessPiece>` which represents the current state of the
    ///             simple_chess board, including all pieces and their positions.
    ///
    /// # Returns
    ///
    /// A `Vec<ChessMoveType>` containing all possible moves for the simple_chess piece from its current
    /// position, based on the rules for the specific type of simple_chess piece.
    ///
    /// # Examples
    ///
    /// ```
    /// use simple_chess::piece::{ChessPiece, PieceType};
    /// use simple_chess::Color;
    /// use game_board::Board;
    ///
    /// let white_king = ChessPiece::new(PieceType::King, Color::White);
    /// let board = Board::build(8, 8).unwrap();
    /// let moves = white_king.possible_moves((0, 4), &board, None);
    /// assert_eq!(5, moves.len());
    /// ```
    pub fn possible_moves(
        &self,
        position: (usize, usize),
        board: &Board<ChessPiece>,
        last_move: Option<&ChessMoveType>,
    ) -> Vec<ChessMoveType> {
        match self.piece_type {
            PieceType::King => king::possible_moves(self.color, position, board),
            PieceType::Queen => queen::possible_moves(self.color, position, board),
            PieceType::Rook => rook::possible_moves(self.color, position, board),
            PieceType::Bishop => bishop::possible_moves(self.color, position, board),
            PieceType::Knight => knight::possible_moves(self.color, position, board),
            PieceType::Pawn => pawn::possible_moves(self.color, position, board, last_move),
        }
    }
}

impl Display for ChessPiece {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_utf_str())
    }
}
