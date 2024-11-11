use std::hint::black_box;
use rand::Rng;
use chess::Game;
use chess::game_analyser::get_game_state;
use chess::game_state::GameState::*;
use chess::PieceType::{Pawn, Queen};

fn main() {
    let mut game = Game::new_chess_game();

    println!("{}", game.board);
    loop {
        let (state, moves) = get_game_state(&game);

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
                // for m in &moves {
                //     println!("{m}");
                // }
            }
            InProgress => {
                println!("Game is in progress");
                // for m in &moves {
                //     println!("{m}");
                // }
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


        println!("{}", game.board);

        game.change_turn(next_move);
        
        if game.turn_number > 400 {
            println!("game went over 400 moves");
            break;
        }
    }
}
