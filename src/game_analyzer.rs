use crate::chess_piece::PieceType;
use crate::color::Color;
use crate::ChessGame;

enum GameState {
    Checkmate { winner: Color },
    Stalemate,
    InProgress,
}

struct GameAnalysis {
    game_state: GameState,
    legal_moves: Vec<String>,
}

/// Analyzes the game state and returns the result.
/// TODO - Implement this function.
/// TODO - Return the game state as a GameState enum. as well as any legal moves that can be made.
pub fn get_game_state(game: ChessGame) -> GameAnalysis {
    GameAnalysis {
        game_state: GameState::InProgress,
        legal_moves: vec![],
    }
}

fn get_legal_moves(game: ChessGame) -> Vec<String> {
    let mut moves = vec![];

    moves.append(&mut get_pawn_legal_moves(game));

    moves
}

fn get_pawn_legal_moves(game: ChessGame) -> Vec<String> {
    let mut moves = vec![];

    for (x, row) in game.board.get_board().iter().enumerate() {
        for (y, square) in row.iter().enumerate() {
            if let Some(piece) = square {
                if piece.get_color() == game.current_turn {
                    if piece.get_piece_type() == PieceType::Pawn {
                        // moves.push(format!("Pawn at ({}, {})", x, y));
                    }
                }
            }
        }
    }

    moves
}
