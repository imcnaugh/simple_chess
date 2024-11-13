use chess::chess_move::ChessMove;
use chess::game_analyser::get_game_state;
use chess::game_state::GameState::*;
use chess::PieceType::{Bishop, Knight, Pawn, Queen, Rook};
use chess::{Color, Game, PieceType};
use rand::Rng;
use std::io::Write;

fn main() {
    let mut game = Game::new_chess_game();

    loop {
        let (state, moves) = get_game_state(&game);

        clear_console();
        println!("{}", game.get_board());
        if let Some(last_move) = game.get_moves().last() {
            println!("{last_move}");
        }

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

        let mut next_move = match game.current_turn {
            Color::White => print_and_get_next_move(moves),
            Color::Black => pick_random_move(moves),
        };

        if next_move.piece.piece_type == Pawn
            && (next_move.new_position.1 == 0
                || next_move.new_position.1 == game.board.get_height() - 1)
        {
            let promotion_piece = match game.current_turn {
                Color::White => promote_pawn_selection(),
                Color::Black => Queen,
            };
            next_move.piece.piece_type = promotion_piece;
        }

        if let Some((taken_col, taken_row)) = next_move.taken_piece_position {
            game.get_board_mut().remove_piece(taken_col, taken_row);
        }

        game.get_board_mut().place_piece(
            next_move.piece,
            next_move.new_position.0,
            next_move.new_position.1,
        );
        game.get_board_mut()
            .remove_piece(next_move.original_position.0, next_move.original_position.1);

        game.change_turn(next_move);
    }
}

fn promote_pawn_selection() -> PieceType {
    let options = [Bishop, Knight, Rook, Queen];
    for (index, piece_type) in options.iter().enumerate() {
        println!("{index}: {:?}", piece_type);
    }

    let mut i = String::new();
    std::io::stdin().read_line(&mut i).expect("TODO: panic message");

    let i: usize = i.trim().parse().expect("Please enter a valid index.");

    options[i]
}

fn pick_random_move(moves: Vec<ChessMove>) -> ChessMove {
    let random_move_index = rand::thread_rng().gen_range(0..moves.len());
    moves[random_move_index]
}

fn print_and_get_next_move(moves: Vec<ChessMove>) -> ChessMove {
    for (index, m) in moves.iter().enumerate() {
        println!("{index}: {m}");
    }

    // wait for the user to press the enter key
    let mut i = String::new();
    std::io::stdin().read_line(&mut i).expect("TODO: panic message");

    let i: usize = i.trim().parse().expect("Please enter a valid index.");

    moves[i]
}

fn clear_console() {
    // Print the escape code to clear the console
    print!("\x1B[2J\x1B[1;1H");
    // Flush to ensure it executes immediately
    std::io::stdout().flush().unwrap();
}
