use crate::piece::ChessPiece;
use crate::{ChessGame, ChessMoveType, Color};
use game_board::Board;
use crate::piece::PieceType::King;

pub fn get_legal_moves(mut game: ChessGame) -> Vec<ChessMoveType> {
    let current_turn = game.get_current_players_turn();
    let last_move = game.get_last_move();
    let board = game.get_board();

    let all_moves = get_all_moves_for_color(current_turn.opposite(), board, last_move);
    all_moves.into_iter().filter(|possible_move| {
        let board = game.get_board_mut();
        possible_move.make_move(board);
        let in_check = is_in_check(current_turn, &board, Some(possible_move));
        possible_move.undo_move(board);
        !in_check
    }).collect::<Vec<ChessMoveType>>()
}

fn is_in_check(color: Color, board: &Board<ChessPiece>, last_move: Option<&ChessMoveType>) -> bool {
    for row in 0..board.get_height() {
        for col in 0..board.get_width() {
            if let Some(piece) = board.get_piece_at_space(col, row) {
                if piece.get_color() == color.opposite() {
                    let moves = piece.possible_moves((col, row), board, last_move);
                    for m in moves {
                        match m {
                            ChessMoveType::Move { taken_piece, .. } => {
                                if let Some(taken_piece) = taken_piece {
                                    if taken_piece.get_piece_type() == King {
                                        return true
                                    }
                                }
                            }
                            ChessMoveType::EnPassant { taken_piece, .. } => {
                                if taken_piece.get_piece_type() == King {
                                    return true
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    false
}

fn get_all_moves_for_color(color: Color, board: &Board<ChessPiece>, last_move: Option<&ChessMoveType>) -> Vec<ChessMoveType> {
    let mut moves: Vec<ChessMoveType> = Vec::new();

    for row in 0..board.get_height() {
        for col in 0..board.get_width() {
            if let Some(piece) = board.get_piece_at_space(col, row) {
                if piece.get_color() == color {
                    moves.append(&mut piece.possible_moves((col, row), board, last_move));
                }
            }
        }
    }

    moves
}

