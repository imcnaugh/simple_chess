use crate::piece::ChessPiece;
use game_board::Board;

#[derive(Debug, PartialEq)]
pub enum ChessMoveType {
    Move {
        original_position: (usize, usize),
        new_position: (usize, usize),
        piece: ChessPiece,
        taken_piece: Option<ChessPiece>,
        promotion: Option<ChessPiece>,
    },
    EnPassant {
        original_position: (usize, usize),
        new_position: (usize, usize),
        piece: ChessPiece,
        taken_piece: ChessPiece,
        taken_piece_position: (usize, usize),
        promotion: Option<ChessPiece>,
    },
    Castle {
        rook_original_position: (usize, usize),
        rook_new_position: (usize, usize),
        king_original_position: (usize, usize),
        king_new_position: (usize, usize),
    }
}

impl ChessMoveType {
    pub fn make_move(&self, board: &mut Board<ChessPiece>) {
        match self {
            ChessMoveType::Move {
                original_position,
                new_position,
                promotion,
                ..
            } => {
                let piece = board.remove_piece(original_position.0, original_position.1).unwrap();
                Self::place_piece(board, new_position, piece, *promotion);
            }
            ChessMoveType::EnPassant {
                original_position,
                new_position,
                taken_piece_position,
                promotion,
                ..
            } => {
                let moving_pawn = board.remove_piece(original_position.0, original_position.1).unwrap();
                board.remove_piece(taken_piece_position.0, taken_piece_position.1);
                Self::place_piece(board, new_position, moving_pawn, *promotion);
            }
            ChessMoveType::Castle {
                rook_original_position, rook_new_position, king_original_position, king_new_position
            } => {
                let rook = board.remove_piece(rook_original_position.0, rook_original_position.1).unwrap();
                let king = board.remove_piece(king_original_position.0, king_original_position.1).unwrap();
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
                rook_original_position, rook_new_position, king_original_position, king_new_position
            } => {
                let rook = board.remove_piece(rook_new_position.0, rook_new_position.1).unwrap();
                let king = board.remove_piece(king_new_position.0, king_new_position.1).unwrap();
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
