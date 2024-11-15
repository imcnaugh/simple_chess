use chess::game_analyser::get_game_state;
use chess::game_state::GameState::*;
use chess::{Game};
use rand::Rng;
use std::io::Write;

fn main() {
    let mut game = Game::new_chess_game();

    loop {
        let (state, moves) = get_game_state(&game);

        clear_console();
        if let Some(last_move) = game.get_moves().last() {
            println!("{last_move}");
        }
        println!("{}", game.get_board());

        match state {
            Checkmate => {
                println!(
                    "Game ends in Checkmate {:?} wins in {} moves",
                    game.current_turn.opposite_color(),
                    game.turn_number
                );
                break;
            }
            Stalemate => {
                println!("Game ends in Stalemate in {} moves", game.turn_number);
                break;
            }
            Check => {
                println!("{:?} is in check", game.current_turn);
            }
            InProgress => {
                // for m in &moves {
                //     println!("{m}");
                // }
            }
            InsufficientMaterial => {
                println!(
                    "Game is over due to Insufficient Material in {} moves",
                    game.turn_number
                );
                break;
            }
            FiftyMoveRule => {
                println!(
                    "Game ends in draw by 50 move rule at move {}",
                    game.turn_number
                );
                break;
            }
        }

        let move_count = moves.len();
        let random_move_index = rand::thread_rng().gen_range(0..move_count);
        let next_move = &moves[random_move_index];

        game.change_turn(*next_move);
    }
}

// fn print_and_get_next_move(moves: Vec<ChessMove>) -> ChessMove {
//     for (index, m) in moves.iter().enumerate() {
//         println!("{index}: {m}");
//     }
//
//     // wait for the user to press the enter key
//     let mut i = String::new();
//     std::io::stdin()
//         .read_line(&mut i)
//         .expect("TODO: panic message");
//
//     let i: usize = i.trim().parse().expect("Please enter a valid index.");
//
//     moves[i]
// }

fn clear_console() {
    // Print the escape code to clear the console
    print!("\x1B[2J\x1B[1;1H");
    // Flush to ensure it executes immediately
    std::io::stdout().flush().unwrap();
}
