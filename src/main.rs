use chess_game::chess_game_state_analyzer::GameState;
use chess_game::{ChessGame, ChessMoveType, Color};
use rand::seq::SliceRandom;
use rand::thread_rng;

fn main() {
    let mut game = ChessGame::new();
    println!("Welcome to Chess Game!");
    println!("Its {:?}'s turn", game.get_current_players_turn());

    loop {
        let state = game.get_game_state();
        println!("{}", game.get_board());

        let next_move = match state {
            GameState::InProgress { legal_moves, turn } => {
                println!("play on, Its {:?}'s turn.", turn);
                match turn {
                    Color::White => list_moves_and_select_one(legal_moves),
                    Color::Black => pick_random_move(legal_moves),
                }
            }
            GameState::Check { legal_moves, turn } => {
                println!("Check! It's {:?}'s turn.", turn);
                match turn {
                    Color::White => list_moves_and_select_one(legal_moves),
                    Color::Black => pick_random_move(legal_moves),
                }
            }
            GameState::Checkmate { winner } => {
                println!("Checkmate! {:?} wins!", winner);
                break;
            }
            GameState::Stalemate => {
                println!("Stalemate!");
                break;
            }
        };

        game.make_move(next_move);
    }
}

fn list_moves_and_select_one(moves: Vec<ChessMoveType>) -> ChessMoveType {
    for (index, chess_move) in moves.iter().enumerate() {
        println!("{}. {}", index, chess_move);
    }

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    let input: usize = input.trim().parse().expect("Please enter a valid number");

    moves[input]
}

fn pick_random_move(moves: Vec<ChessMoveType>) -> ChessMoveType {
    let mut rng = thread_rng();
    *moves.choose(&mut rng).expect("No moves given")
}