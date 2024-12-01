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
}

impl ChessMoveType {
    pub fn make_move(&self, board: &mut Board<ChessPiece>) {
        match self {
            ChessMoveType::Move {
                original_position,
                new_position,
                piece,
                promotion,
                ..
            } => {
                board.remove_piece(original_position.0, original_position.1);
                Self::place_piece(board, new_position, piece, promotion);
            }
            ChessMoveType::EnPassant {
                original_position,
                new_position,
                piece,
                taken_piece_position,
                promotion,
                ..
            } => {
                board.remove_piece(original_position.0, original_position.1);
                board.remove_piece(taken_piece_position.0, taken_piece_position.1);
                Self::place_piece(board, new_position, piece, promotion);
            }
        }
    }

    fn place_piece(
        board: &mut Board<ChessPiece>,
        new_position: &(usize, usize),
        piece: &ChessPiece,
        promotion: &Option<ChessPiece>,
    ) {
        match promotion {
            Some(promotion) => board.place_piece(*promotion, new_position.0, new_position.1),
            None => board.place_piece(*piece, new_position.0, new_position.1),
        }
    }
}
