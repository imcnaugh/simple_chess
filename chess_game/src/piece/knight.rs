use crate::chess_move::ChessMoveType;
use crate::piece::{ChessPiece, PieceType};
use crate::Color;
use game_board::Board;

pub fn as_utf_str(color: Color) -> &'static str {
    match color {
        Color::White => "♘",
        Color::Black => "♞",
    }
}

pub fn as_fen_char(color: Color) -> char {
    match color {
        Color::White => 'N',
        Color::Black => 'n',
    }
}

pub fn possible_moves(
    color: Color,
    position: (usize, usize),
    board: &Board<ChessPiece>,
) -> Vec<ChessMoveType> {
    let mut possible_moves: Vec<ChessMoveType> = Vec::new();

    let moves = [
        (1i32, 2),
        (1, -2),
        (-1, 2),
        (-1, -2),
        (2, 1),
        (2, -1),
        (-2, 1),
        (-2, -1),
    ];

    for mv in moves.iter() {
        let x = position.0 as i32 + mv.0;
        let y = position.1 as i32 + mv.1;

        if x >= 0 && x < board.get_width() as i32 && y >= 0 && y < board.get_height() as i32 {
            if let Some(piece) = board.get_piece_at_space(x as usize, y as usize) {
                if piece.get_color() != color {
                    possible_moves.push(ChessMoveType::Move {
                        original_position: position,
                        new_position: (x as usize, y as usize),
                        piece: ChessPiece::new(PieceType::Knight, color),
                        taken_piece: Some(*piece),
                        promotion: None,
                    });
                }
            } else {
                possible_moves.push(ChessMoveType::Move {
                    original_position: position,
                    new_position: (x as usize, y as usize),
                    piece: ChessPiece::new(PieceType::Knight, color),
                    taken_piece: None,
                    promotion: None,
                });
            }
        }
    }

    possible_moves
}
