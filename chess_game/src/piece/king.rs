use crate::chess_move::ChessMoveType;
use crate::piece::ChessPiece;
use crate::piece::PieceType::King;
use crate::Color;
use game_board::Board;

pub fn as_utf_str(color: Color) -> &'static str {
    match color {
        Color::White => "♔",
        Color::Black => "♚",
    }
}

pub fn as_fen_char(color: Color) -> char {
    match color {
        Color::White => 'K',
        Color::Black => 'k',
    }
}

pub fn possible_moves(
    color: Color,
    position: (usize, usize),
    board: Board<ChessPiece>,
) -> Vec<ChessMoveType> {
    let mut possible_moves: Vec<ChessMoveType> = Vec::new();
    let moves = [
        (0i32, 1),
        (0, -1),
        (1, 0),
        (-1, 0),
        (1, 1),
        (1, -1),
        (-1, 1),
        (-1, -1),
    ];
    for mv in moves.iter() {
        let new_col = position.0 as i32 + mv.0;
        let new_row = position.1 as i32 + mv.1;

        if new_col < 0
            || new_col >= board.get_width() as i32
            || new_row < 0
            || new_row >= board.get_height() as i32
        {
            continue;
        }

        let new_col = new_col as usize;
        let new_row = new_row as usize;

        match board.get_piece_at_space(new_col, new_row) {
            Some(p) => {
                if p.get_color() != color {
                    possible_moves.push(ChessMoveType::Move {
                        original_position: position,
                        new_position: (new_col, new_row),
                        piece: ChessPiece::new(King, color),
                        taken_piece: Some(*p),
                        promotion: None,
                    });
                }
            }
            None => possible_moves.push(ChessMoveType::Move {
                original_position: position,
                new_position: (new_col, new_row),
                piece: ChessPiece::new(King, color),
                taken_piece: None,
                promotion: None,
            }),
        }
    }

    possible_moves
}
