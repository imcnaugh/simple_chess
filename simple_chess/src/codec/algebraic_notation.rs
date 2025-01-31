use game_board::Board;
use crate::ChessMoveType;
use crate::piece::ChessPiece;

pub fn encode_move_as_algebraic_notation(chess_move: &ChessMoveType, board: Board<ChessPiece>) -> String {
    chess_move.to_string()
}