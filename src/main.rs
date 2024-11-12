use std::io::Write;
use rand::Rng;
use chess::Game;
use chess::game_analyser::get_game_state;
use chess::game_state::GameState::*;
use chess::PieceType::{Pawn, Queen};

fn main() {
    let mut game = Game::new_chess_game();

    loop {
        let (state, moves) = get_game_state(&game);

        clear_console();
        println!("{}", game.get_board());

        match state {
            Checkmate => {
                println!("Game ends in Checkmate {:?} wins in {} moves", game.current_turn.opposite_color(), game.turn_number);
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
            },
            InsufficientMaterial => {
                println!("Game is over due to Insufficient Material in {} moves", game.turn_number);
                break;
            },
            FiftyMoveRule => {
                println!("Game ends in draw by 50 move rule at move {}", game.turn_number);
                break;
            }
        }

        // pick a random move from moves
        let random_move_index = rand::thread_rng().gen_range(0..moves.len());
        let mut next_move = moves[random_move_index];

        println!("{next_move}");
        if next_move.piece.piece_type == Pawn && (next_move.new_position.1 == 0 || next_move.new_position.1 == game.board.get_height()-1) {
            println!("Pawn promotes to a queen");
            next_move.piece.piece_type = Queen;
        }
        game.get_board_mut().place_piece(next_move.piece, next_move.new_position.0, next_move.new_position.1);
        game.get_board_mut().remove_piece(next_move.original_position.0, next_move.original_position.1);

        game.change_turn(next_move);

        // wait 1 second
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}

fn clear_console() {
    // Print the escape code to clear the console
    print!("\x1B[2J\x1B[1;1H");
    // Flush to ensure it executes immediately
    std::io::stdout().flush().unwrap();
}
