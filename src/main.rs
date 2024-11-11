use chess::Game;
use chess::game_analyser::get_game_state;
use chess::game_state::GameState::*;

fn main() {
    let mut game = Game::new_chess_game();

    loop {
        println!("{}", game.board);

        let (state, moves) = get_game_state(&mut game);

        match state {
            Checkmate => {
                println!("Game ends in Checkmate {:?} wins", game.current_turn.opposite_color());
                break;
            }
            Stalemate => {
                println!("Game ends in Stalemate");
                break;
            }
            Check => {
                for m in &moves {
                    println!("{m}");
                }
            }
            InProgress => {
                for m in &moves {
                    println!("{m}");
                }
            }
        }

        if let Some(next_move) = moves.first() {
            game.make_move(next_move);
        }
    }
}
