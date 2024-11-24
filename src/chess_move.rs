use crate::chess_board::GameBoard;
use crate::chess_board_square::SquareId;
use crate::ChessPiece;
use std::fmt;
use std::fmt::Formatter;

#[derive(Copy, Clone)]
pub enum ChessMoveType {
    Move {
        original_position: SquareId,
        new_position: SquareId,
        piece: ChessPiece,
        taken_piece: Option<ChessPiece>,
        promotion: Option<ChessPiece>,
    },
    EnPassant {
        original_position: SquareId,
        new_position: SquareId,
        piece: ChessPiece,
        taken_piece: ChessPiece,
        taken_piece_position: SquareId,
    },
    Castle {
        king: ChessPiece,
        original_king_position: SquareId,
        new_king_position: SquareId,
        rook: ChessPiece,
        original_rook_position: SquareId,
        new_rook_position: SquareId,
    },
}

impl ChessMoveType {
    pub fn make_move(&self, board: &mut GameBoard) {
        match self {
            ChessMoveType::Move {
                original_position,
                new_position,
                piece,
                promotion,
                ..
            } => {
                board.remove_piece(original_position.get_column(), original_position.get_row());
                if let Some(promotion) = promotion {
                    board.place_piece(
                        *promotion,
                        new_position.get_column(),
                        new_position.get_row(),
                    );
                } else {
                    board.place_piece(*piece, new_position.get_column(), new_position.get_row());
                }
            }
            ChessMoveType::EnPassant {
                original_position,
                new_position,
                piece,
                taken_piece_position,
                ..
            } => {
                board.remove_piece(
                    taken_piece_position.get_column(),
                    taken_piece_position.get_row(),
                );
                board.remove_piece(original_position.get_column(), original_position.get_row());
                board.place_piece(*piece, new_position.get_column(), new_position.get_row());
            }
            ChessMoveType::Castle {
                king,
                original_king_position,
                new_king_position,
                rook,
                original_rook_position,
                new_rook_position,
            } => {
                board.remove_piece(
                    original_king_position.get_column(),
                    original_king_position.get_row(),
                );
                board.remove_piece(
                    original_rook_position.get_column(),
                    original_rook_position.get_row(),
                );
                board.place_piece(
                    *king,
                    new_king_position.get_column(),
                    new_king_position.get_row(),
                );
                board.place_piece(
                    *rook,
                    new_rook_position.get_column(),
                    new_rook_position.get_row(),
                );
            }
        };
    }

    pub fn get_standard_algebraic_notation(&self) -> String {
        match self {
            ChessMoveType::Move {
                original_position,
                new_position,
                piece,
                taken_piece,
                promotion,
            } => {
                let promotion_string = match promotion {
                    None => String::new(),
                    Some(piece) => format!("={}", piece.get_notation_char()),
                };

                let piece_moving = piece.get_notation_char();

                format!(
                    "{}{}{}{}{}",
                    piece_moving,
                    original_position,
                    if taken_piece.is_some() { "x" } else { "" },
                    new_position,
                    promotion_string
                )
            }
            ChessMoveType::EnPassant {
                original_position,
                new_position,
                ..
            } => {
                format!("{}x{}", original_position, new_position)
            }
            ChessMoveType::Castle {
                original_rook_position,
                ..
            } => {
                if original_rook_position.get_column() == 0 {
                    String::from("O-O-O")
                } else {
                    String::from("O-O")
                }
            }
        }
    }
}

impl fmt::Display for ChessMoveType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ChessMoveType::Move {
                original_position,
                new_position,
                piece,
                taken_piece,
                promotion,
            } => {
                let promotion_message = match promotion {
                    Some(promotion_piece) => {
                        format!(" and promotes to {:?}", promotion_piece.piece_type)
                    }
                    None => String::new(),
                };

                if let Some(taken_piece) = taken_piece {
                    write!(
                        f,
                        "{:?} {:?} takes {:?} at {} from {} {}",
                        piece.color,
                        piece.piece_type,
                        taken_piece.piece_type,
                        new_position,
                        original_position,
                        promotion_message
                    )
                } else {
                    write!(
                        f,
                        "{:?} {:?} moves to {} from {} {}",
                        piece.color,
                        piece.piece_type,
                        new_position,
                        original_position,
                        promotion_message
                    )
                }
            }
            ChessMoveType::EnPassant {
                original_position,
                new_position,
                piece,
                taken_piece,
                ..
            } => {
                write!(
                    f,
                    "{:?} {:?} at {}, En Passant's {:?} at {}",
                    piece.color,
                    piece.piece_type,
                    original_position,
                    taken_piece.piece_type,
                    new_position
                )
            }
            ChessMoveType::Castle {
                original_rook_position,
                ..
            } => {
                let castle_direction = if original_rook_position.get_column() == 0 {
                    "Queen side"
                } else {
                    "King side"
                };
                write!(f, "Castles {}", castle_direction)
            }
        }
    }
}
