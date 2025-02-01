use game_board::Board;
use rand::seq::SliceRandom;
use rand::thread_rng;
use simple_chess::chess_game_state_analyzer::GameState;
use simple_chess::piece::ChessPiece;
use simple_chess::{ChessGame, ChessMoveType, Color};
use simple_chess::codec::algebraic_notation::{encode_move_as_algebraic_notation};
use simple_chess::Color::{Black, White};

fn main() {
    let mut game = ChessGame::new();

    loop {
        let state = game.get_game_state();

        if let Some(_reason) = game.can_claim_draw() {
            break;
        }

        let next_move = match state {
            GameState::InProgress { legal_moves, turn } => match turn {
                Color::White => pick_random_move(legal_moves),
                Color::Black => pick_random_move(legal_moves),
            },
            GameState::Check { legal_moves, turn } => {
                match turn {
                    Color::White => pick_random_move(legal_moves),
                    Color::Black => pick_random_move(legal_moves),
                }
            }
            GameState::Checkmate { winner: _winner } => {
                break;
            }
            GameState::Stalemate => {
                break;
            }
        };

        game.make_move(next_move);
    }

    let mut current_turn = White;
    let mut current_turn_number = 1;
    for m in game.get_moves() {
        if current_turn == White {
            print!("{}.", current_turn_number);
        }

        let move_str = encode_move_as_algebraic_notation(m);
        print!("{}", move_str);

        if current_turn == Black {
            print!("\n");
            current_turn_number = current_turn_number+1;
        } else {
            print!(" ");
        }
        current_turn = current_turn.opposite();
    }
}

fn _list_moves_and_select_one(
    moves: Vec<ChessMoveType>,
    board: &Board<ChessPiece>,
) -> ChessMoveType {
    println!("{}", board);
    for (index, chess_move) in moves.iter().enumerate() {
        println!("{}. {}", index, chess_move);
    }

    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let input: usize = input.trim().parse().expect("Please enter a valid number");

    moves[input]
}

fn pick_random_move(moves: Vec<ChessMoveType>) -> ChessMoveType {
    let mut rng = thread_rng();
    *moves.choose(&mut rng).expect("No moves given")
}
