use crate::piece::ChessPiece;
use crate::{ChessGame, ChessMoveType, Color};
use game_board::Board;

pub fn get_legal_moves(game: ChessGame) -> Vec<ChessMoveType> {
    let current_turn = game.get_current_players_turn();

    let board = game.get_board();

    let mut legal_moves: Vec<ChessMoveType> = Vec::new();

    for row in 0..board.get_height() {
        for col in 0..board.get_width() {
            if let Some(piece) = board.get_piece_at_space(col, row) {
                if piece.get_color() == current_turn {
                    let moves: Vec<ChessMoveType> = piece
                        .possible_moves((col, row), board, game.get_last_move())
                        .iter()
                        .filter(|m| -> bool {
                            let mut board = game.get_board().clone();
                            *m.make_move(&mut board);
                            is_in_check(current_turn.opposite(), board)
                        }).map(|&m| -> ChessMoveType {
                        m
                    }).collect();
                    legal_moves.extend(moves);
                }
            }
        }
    }
    legal_moves
}

fn is_in_check(color: Color, board: &Board<ChessPiece>) -> bool {
    false
}
