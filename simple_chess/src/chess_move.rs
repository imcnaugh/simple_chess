use crate::piece::ChessPiece;
use game_board::{get_square_name_from_row_and_col, Board};
use std::fmt::{Display, Formatter};

/// Represents different types of simple_chess moves.
///
/// This enum captures the state and specific details of various types of legal simple_chess moves.
/// It provides functionality to apply and undo these moves on a simple_chess board.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ChessMoveType {
    /// A regular piece move which might include capturing an opponent's piece or promoting a pawn.
    ///
    /// Fields:
    /// - `original_position`: (usize, usize) - The starting position of the simple_chess piece.
    /// - `new_position`: (usize, usize) - The destination position of the simple_chess piece.
    /// - `piece`: ChessPiece - The simple_chess piece being moved.
    /// - `taken_piece`: Option<ChessPiece> - The opponent's piece captured during this move, if any.
    /// - `promotion`: Option<ChessPiece> - The piece type a pawn is promoted to, if applicable.
    Move {
        original_position: (usize, usize),
        new_position: (usize, usize),
        piece: ChessPiece,
        taken_piece: Option<ChessPiece>,
        promotion: Option<ChessPiece>,
    },

    /// A special pawn capture move where a pawn captures another pawn that has moved
    /// two squares forward from its starting position on the previous turn.
    ///
    /// Fields:
    /// - `original_position`: (usize, usize) - The starting position of the pawn.
    /// - `new_position`: (usize, usize) - The destination position of the pawn after capturing.
    /// - `piece`: ChessPiece - The pawn performing the en passant capture.
    /// - `taken_piece`: ChessPiece - The opponent's pawn being captured.
    /// - `taken_piece_position`: (usize, usize) - The position of the captured pawn.
    /// - `promotion`: Option<ChessPiece> - The piece type if the pawn is promoted, if applicable.
    EnPassant {
        original_position: (usize, usize),
        new_position: (usize, usize),
        piece: ChessPiece,
        taken_piece: ChessPiece,
        taken_piece_position: (usize, usize),
        promotion: Option<ChessPiece>,
    },

    /// A special move involving the king and a rook where both pieces move simultaneously.
    ///
    /// Fields:
    /// - `rook_original_position`: (usize, usize) - The starting position of the rook.
    /// - `rook_new_position`: (usize, usize) - The destination position of the rook.
    /// - `king_original_position`: (usize, usize) - The starting position of the king.
    /// - `king_new_position`: (usize, usize) - The destination position of the king.
    Castle {
        rook_original_position: (usize, usize),
        rook_new_position: (usize, usize),
        king_original_position: (usize, usize),
        king_new_position: (usize, usize),
    },
}

impl ChessMoveType {
    /// Applies the current move to the simple_chess board.
    ///
    /// This method performs the actual move on the board by repositioning the involved
    /// pieces according to the move type.
    ///
    /// # Arguments
    ///
    /// * `board` - A mutable reference to the simple_chess board on which the move will be applied.
    ///
    /// # Examples
    ///
    /// ```
    /// use simple_chess::ChessMoveType;
    /// use simple_chess::Color::White;
    /// use simple_chess::piece::ChessPiece;
    /// use simple_chess::piece::PieceType::Pawn;
    /// use game_board::Board;
    /// let mut board = Board::build(8, 8).unwrap();
    /// let original_position = (0, 1);
    ///
    /// board.place_piece(ChessPiece::new(Pawn, White), original_position.0, original_position.1);
    ///
    /// let game_move = ChessMoveType::Move {
    ///     original_position,
    ///     new_position: (0, 2),
    ///     piece: *board.get_piece_at_space(original_position.0, original_position.1).unwrap(),
    ///     taken_piece: None,
    ///     promotion: None,
    /// };
    ///
    /// assert!(board.get_piece_at_space(0, 2).is_none());
    /// game_move.make_move(&mut board);
    ///
    /// assert!(board.get_piece_at_space(original_position.0, original_position.1).is_none());
    /// assert_eq!(Pawn, board.get_piece_at_space(0, 2).unwrap().get_piece_type());
    /// assert_eq!(White, board.get_piece_at_space(0, 2).unwrap().get_color());
    /// ```
    pub fn make_move(&self, board: &mut Board<ChessPiece>) {
        match self {
            ChessMoveType::Move {
                original_position,
                new_position,
                promotion,
                ..
            } => {
                let piece = board
                    .remove_piece(original_position.0, original_position.1)
                    .unwrap();
                Self::place_piece(board, new_position, piece, *promotion);
            }
            ChessMoveType::EnPassant {
                original_position,
                new_position,
                taken_piece_position,
                promotion,
                ..
            } => {
                let moving_pawn = board
                    .remove_piece(original_position.0, original_position.1)
                    .unwrap();
                board.remove_piece(taken_piece_position.0, taken_piece_position.1);
                Self::place_piece(board, new_position, moving_pawn, *promotion);
            }
            ChessMoveType::Castle {
                rook_original_position,
                rook_new_position,
                king_original_position,
                king_new_position,
            } => {
                let rook = board
                    .remove_piece(rook_original_position.0, rook_original_position.1)
                    .unwrap();
                let king = board
                    .remove_piece(king_original_position.0, king_original_position.1)
                    .unwrap();
                board.place_piece(rook, rook_new_position.0, rook_new_position.1);
                board.place_piece(king, king_new_position.0, king_new_position.1);
            }
        }
    }

    pub fn undo_move(&self, board: &mut Board<ChessPiece>) {
        match self {
            ChessMoveType::Move {
                original_position,
                new_position,
                piece,
                taken_piece,
                ..
            } => {
                Self::place_piece(board, original_position, *piece, None);
                board.remove_piece(new_position.0, new_position.1);
                if let Some(taken_piece) = taken_piece {
                    Self::place_piece(board, new_position, *taken_piece, None);
                }
            }
            ChessMoveType::EnPassant {
                original_position,
                new_position,
                piece,
                taken_piece,
                taken_piece_position,
                ..
            } => {
                Self::place_piece(board, original_position, *piece, None);
                Self::place_piece(board, taken_piece_position, *taken_piece, None);
                board.remove_piece(new_position.0, new_position.1);
            }
            ChessMoveType::Castle {
                rook_original_position,
                rook_new_position,
                king_original_position,
                king_new_position,
            } => {
                let rook = board
                    .remove_piece(rook_new_position.0, rook_new_position.1)
                    .unwrap();
                let king = board
                    .remove_piece(king_new_position.0, king_new_position.1)
                    .unwrap();
                board.place_piece(rook, rook_original_position.0, rook_original_position.1);
                board.place_piece(king, king_original_position.0, king_original_position.1);
            }
        }
    }

    fn place_piece(
        board: &mut Board<ChessPiece>,
        new_position: &(usize, usize),
        piece: ChessPiece,
        promotion: Option<ChessPiece>,
    ) {
        match promotion {
            Some(promotion) => board.place_piece(promotion, new_position.0, new_position.1),
            None => board.place_piece(piece, new_position.0, new_position.1),
        }
    }
}

impl Display for ChessMoveType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ChessMoveType::Move {
                original_position,
                new_position,
                piece,
                taken_piece,
                promotion,
            } => {
                let take_string = if let Some(taken_piece) = taken_piece {
                    format!("takes {:?}", taken_piece.get_piece_type())
                } else {
                    String::from("moves")
                };

                let promotion_string = if let Some(promotion) = promotion {
                    format!("promotes to {:?}", promotion.get_piece_type())
                } else {
                    String::new()
                };

                write!(
                    f,
                    "{:?} at {} {} at {} {}",
                    piece.get_piece_type(),
                    get_square_name_from_row_and_col(original_position.0, original_position.1),
                    take_string,
                    get_square_name_from_row_and_col(new_position.0, new_position.1),
                    promotion_string
                )
            }
            ChessMoveType::EnPassant {
                original_position, ..
            } => {
                write!(f, "en passant from {:?}", original_position)
            }
            ChessMoveType::Castle { .. } => {
                write!(f, "Castle")
            }
        }
    }
}
