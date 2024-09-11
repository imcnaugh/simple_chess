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
