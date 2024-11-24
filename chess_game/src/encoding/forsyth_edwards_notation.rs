use game_board::Board;
use crate::chess_game::ChessGame;
use crate::Color;
use crate::piece::ChessPiece;

pub fn encode_game_as_string(game: &ChessGame) -> String {
    format!("{} {}",
            get_board_as_fen_string(game),
            get_current_turn_char(game)
    )
}

fn get_board_as_fen_string(game: &ChessGame) -> String {
    let board = game.get_board();

    let board_as_fen_string: String = (0..board.get_height()).rev()
        .map(|rank| encode_row(board, rank))
        .collect().join("/");
    board_as_fen_string
}

fn encode_row(board: &Board<dyn ChessPiece>, row: usize) -> String {
    let mut result = String::new();
    
    let mut empty_space_counter: usize = 0;
    
    for col in 0..board.get_width() {
        if let Some(piece) = board.get_piece_at_space(col, row) {
            if empty_space_counter != 0 {
                result.push_str(&empty_space_counter.to_string());
                empty_space_counter = 0;
            }
            result.push(piece.get_fen_char());
        } else {
            empty_space_counter+=1;
        }
        if empty_space_counter != 0 {
            result.push_str(&empty_space_counter.to_string());
        }
    }
    result
}

fn get_current_turn_char(game: &ChessGame) -> char {
    let current_turn = match game.get_current_players_turn() {
        Color::White => 'w',
        Color::Black => 'b',
    };
    current_turn
}

fn get_castling_rights(game: &ChessGame) -> String {
    let mut result = String::new();

    if game.
}