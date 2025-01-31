use crate::chess_game::ChessGame;

pub fn encode_game_in_long_algebraic_notation(game: &ChessGame) -> String {
    let mut encoded_game = String::new();

    for mv in game.get_moves() {
        encoded_game.push_str(&format!("{} ", mv.as_long_algebraic_notation()));
    }
    encoded_game
}
