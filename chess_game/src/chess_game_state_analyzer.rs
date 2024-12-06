use crate::chess_game_move_analyzer;
use crate::piece::ChessPiece;
use crate::piece::PieceType::King;
use crate::ChessMoveType::Move;
use crate::{ChessGame, ChessMoveType, Color};
use game_board::Board;

pub enum GameState {
    InProgress {
        legal_moves: Vec<ChessMoveType>,
        turn: Color,
    },
    Check {
        legal_moves: Vec<ChessMoveType>,
        turn: Color,
    },
    Checkmate {
        winner: Color,
    },
    Stalemate,
}

pub fn get_game_state(game: &mut ChessGame) -> GameState {
    let legal_moves = chess_game_move_analyzer::get_legal_moves(game);
    if is_in_check(game.get_current_players_turn(), game.get_board()) {
        if legal_moves.is_empty() {
            GameState::Checkmate {
                winner: game.get_current_players_turn(),
            }
        } else {
            GameState::Check {
                legal_moves,
                turn: game.get_current_players_turn().opposite(),
            }
        }
    } else {
        if legal_moves.is_empty() {
            GameState::Stalemate
        } else {
            GameState::InProgress {
                legal_moves,
                turn: game.get_current_players_turn().opposite(),
            }
        }
    }
}

pub fn is_in_check(color: Color, board: &Board<ChessPiece>) -> bool {
    for row in 0..board.get_height() {
        for col in 0..board.get_width() {
            if let Some(piece) = board.get_piece_at_space(col, row) {
                if piece.get_color() == color.opposite() {
                    let moves = piece.possible_moves((col, row), board, None);
                    for m in moves {
                        match m {
                            Move { taken_piece, .. } => {
                                if let Some(taken_piece) = taken_piece {
                                    if taken_piece.get_piece_type() == King {
                                        return true;
                                    }
                                }
                            }
                            _ => return false,
                        }
                    }
                }
            }
        }
    }
    false
}
