use crate::chess_piece::{ChessPiece, PieceType};
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
pub fn get_game_state(game: &ChessGame) -> GameAnalysis {
    GameAnalysis {
        game_state: GameState::InProgress,
        legal_moves: vec![],
    }
}

fn get_legal_moves(game: &ChessGame) -> Vec<String> {
    let mut moves = vec![];

    moves.append(&mut get_pawn_legal_moves(game));

    moves
}

fn get_pawn_legal_moves(game: &ChessGame) -> Vec<String> {
    let mut moves = vec![];

    let current_turn = &game.current_turn;

    for (x, row) in game.board.get_board().iter().enumerate() {
        for (y, square) in row.iter().enumerate() {
            if let Some(ChessPiece {
                piece_type: PieceType::Pawn { .. },
                color,
            }) = square
            {
                if color == current_turn {
                    if color == &Color::White {
                        if y == 1 {
                            moves.push(format!("{}{}{}{}", x, y, x, y + 2));
                        }
                        moves.push(format!("{}{}{}{}", x, y, x, y + 1));
                    } else {
                        if y == 6 {
                            moves.push(format!("{}{}{}{}", x, y, x, y - 2));
                        }
                        moves.push(format!("{}{}{}{}", x, y, x, y - 1));
                    }
                }
            }
        }
    }

    moves
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_pawn_legal_moves() {
        let game = ChessGame::new();

        let moves = get_pawn_legal_moves(&game);

        println!("{:?}", moves);
    }
}
