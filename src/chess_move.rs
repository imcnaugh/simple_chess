use std::fmt;
use crate::chess_board_square::SquareId;
use crate::{ChessPiece};
use std::fmt::{Display, Formatter};
use crate::chess_board::GameBoard;

#[derive(Copy, Clone)]
pub enum ChessMoveType {
    Move {
        original_position: SquareId,
        new_position: SquareId,
        piece: ChessPiece,
        taken_piece: Option<ChessPiece>,
        promotion: Option<ChessPiece>
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
                    board.place_piece(*promotion, new_position.get_column(), new_position.get_row());
                } else {
                    board.place_piece(*piece, new_position.get_column(), new_position.get_row());
                }
            }
            ChessMoveType::EnPassant {
                original_position,
                new_position,
                piece: _piece,
                taken_piece,
                taken_piece_position,
            } => {
                board.remove_piece(taken_piece_position.get_column(), taken_piece_position.get_row());
                board.remove_piece(original_position.get_column(), original_position.get_row());
                board.place_piece(*taken_piece, new_position.get_column(), new_position.get_row());
            }
            ChessMoveType::Castle {
                king,
                original_king_position,
                new_king_position,
                rook,
                original_rook_position,
                new_rook_position,
            } => {
                board.remove_piece(original_king_position.get_column(), original_king_position.get_row());
                board.remove_piece(original_rook_position.get_column(), original_rook_position.get_row());
                board.place_piece(*king, new_king_position.get_column(), new_king_position.get_row());
                board.place_piece(*rook, new_rook_position.get_column(), new_rook_position.get_row());
            }
        };
    }
}

impl fmt::Display for ChessMoveType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ChessMoveType::Move {
                original_position, new_position, piece, taken_piece, promotion
            } => {
                let promotion_message = match promotion { 
                    Some(promotion_piece) => format!(" and promotes to {:?}", promotion_piece.piece_type),
                    None => String::new(),
                };
                
                if let Some(taken_piece) = taken_piece {
                    write!(f, "{:?} {:?} takes {:?} at {} from {} {}", piece.color, piece.piece_type, taken_piece.piece_type ,new_position, original_position, promotion_message)
                } else {
                    write!(f, "{:?} {:?} moves to {} from {} {}", piece.color, piece.piece_type, new_position, original_position, promotion_message)
                }
            }
            ChessMoveType::EnPassant { original_position, new_position, piece, taken_piece, .. } => {
                write!(f, "{:?} {:?} at {}, En Passant's {:?} at {}", piece.color, piece.piece_type, original_position, taken_piece.piece_type, new_position)
            }
            ChessMoveType::Castle { .. } => {
                write!(f, "Castles")
            }
        }
    }
}

#[derive(Copy, Clone)]
pub struct ChessMove {
    pub original_position: (usize, usize),
    pub new_position: (usize, usize),
    pub piece: ChessPiece,
    pub takes: Option<ChessPiece>,
    pub taken_piece_position: Option<(usize, usize)>,
}

impl ChessMove {
    pub fn build(
        original_pos: (usize, usize),
        new_pos: (usize, usize),
        piece: ChessPiece,
        takes: Option<ChessPiece>,
        taken_piece_position: Option<(usize, usize)>,
    ) -> Self {
        ChessMove {
            original_position: original_pos,
            new_position: new_pos,
            piece,
            takes,
            taken_piece_position,
        }
    }
}

impl Display for ChessMove {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let action = match self.takes {
            Some(piece) => format!("Takes the {:?} {:?} at", piece.color, piece.piece_type),
            None => "Moves to".to_string(),
        };
        let new_position = format!(
            "{}{}",
            (self.new_position.0 as u8 + b'a') as char,
            (self.new_position.1 as u8 + b'1') as char
        );
        let old_position = format!(
            "{}{}",
            (self.original_position.0 as u8 + b'a') as char,
            (self.original_position.1 as u8 + b'1') as char
        );
        write!(
            f,
            "{:?} {:?} {} {} from {}",
            self.piece.color, self.piece.piece_type, action, new_position, old_position
        )
    }
}
