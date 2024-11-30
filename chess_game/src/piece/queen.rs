use crate::chess_move::ChessMoveType;
use crate::piece::{ChessPiece, PieceType};
use crate::Color;
use game_board::Board;

pub fn as_utf_str(color: Color) -> &'static str {
    match color {
        Color::White => "♕",
        Color::Black => "♛",
    }
}

pub fn as_fen_char(color: Color) -> char {
    match color {
        Color::White => 'Q',
        Color::Black => 'q',
    }
}

pub fn possible_moves(
    color: Color,
    position: (usize, usize),
    board: &Board<ChessPiece>,
) -> Vec<ChessMoveType> {
    let mut possible_moves: Vec<ChessMoveType> = Vec::new();
    let directions = [
        (0i32, 1),
        (0, -1),
        (1, 0),
        (-1, 0),
        (1, 1),
        (1, -1),
        (-1, 1),
        (-1, -1),
    ];

    for dir in directions.iter() {
        let mut x = position.0 as i32 + dir.0;
        let mut y = position.1 as i32 + dir.1;
        while x >= 0 && y >= 0 && x < board.get_width() as i32 && y < board.get_height() as i32 {
            if let Some(piece) = board.get_piece_at_space(x as usize, y as usize) {
                if piece.get_color() != color {
                    possible_moves.push(ChessMoveType::Move {
                        original_position: position,
                        new_position: (x as usize, y as usize),
                        piece: ChessPiece::new(PieceType::Queen, color),
                        taken_piece: Some(*piece),
                        promotion: None,
                    });
                }
            } else {
                possible_moves.push(ChessMoveType::Move {
                    original_position: position,
                    new_position: (x as usize, y as usize),
                    piece: ChessPiece::new(PieceType::Queen, color),
                    taken_piece: None,
                    promotion: None,
                });
            }
            x += dir.0;
            y += dir.1;
        }
    }

    possible_moves
}
