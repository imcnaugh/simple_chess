use game_board::{Board, Piece};
use crate::chess_game::ChessGame;

pub fn encode_game_as_string(game: ChessGame) -> String {
    
    format!("")
}

fn encode_row(board: Board, row: usize) -> String {
    let mut result = String::new();
    
    let mut empty_space_counter: usize = 0;
    
    for col in 0..board.get_width() {
        if let Some(piece) = board.get_piece_at_space(col, row) {
            if empty_space_counter != 0 {
                result.push_str(&empty_space_counter.to_string());
                empty_space_counter = 0;
            }
            result.push(piece.);
        } else {
            empty_space_counter+=1;
        }
    }
    result
}