use crate::ChessGame;
use crate::color::Color;

enum GameState {
    Checkmate {
        winner: Color,
    },
    Stalemate,
    InProgress,
}

/// Analyzes the game state and returns the result.
/// TODO - Implement this function.
/// TODO - Return the game state as a GameState enum. as well as any legal moves that can be made.
pub fn get_game_state(game: ChessGame) {

}