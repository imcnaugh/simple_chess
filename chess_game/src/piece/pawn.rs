use game_board::Board;
use crate::chess_move::ChessMoveType;
use crate::Color;
use crate::piece::ChessPiece;

pub fn as_utf_str(color: Color) -> &'static str {
    match color {
        Color::White => "♙",
        Color::Black => "♟",
    }
}

pub fn as_fen_char(color: Color) -> char {
    match color {
        Color::White => 'P',
        Color::Black => 'p',
    }
}

pub fn possible_moves(
    color: Color,
    position: (usize, usize),
    board: Board<ChessPiece>,
    last_move_type: Option<ChessMoveType>,
) -> Vec<ChessMoveType> {
    let mut possible_moves: Vec<ChessMoveType> = Vec::new();

    let forward_direction = match color {
        Color::White => 1,
        Color::Black => -1,
    };

    let promotion_row = match color {
        Color::White => board.get_height() -1,
        Color::Black => 0,
    };

    let next_row = position.1 as i32 + forward_direction;
    if next_row < 0 || next_row > board.get_height() as i32 {
        return possible_moves;
    }

    if board.get_piece_at_space(position.0, next_row as usize).is_none() {

    }

    possible_moves
}