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

pub fn as_pgn_char() -> Option<char> {
    Some('K')
}

pub fn as_binary(color: Color) -> u8 {
    match color {
        Color::White => 0b1010,
        Color::Black => 0b1011,
    }
}

pub fn possible_moves(
    color: Color,
    position: (usize, usize),
    board: &Board<ChessPiece>,
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::codec::forsyth_edwards_notation::build_game_from_string;
    use crate::piece::PieceType::Pawn;
    use crate::ChessMoveType::Move;
    use crate::Color::{Black, White};

    #[test]
    fn king_can_move_to_any_open_space_around_him() {
        let white_king = ChessPiece::new(King, Color::White);
        let game = build_game_from_string("8/8/8/3K4/8/8/8/8 w - - 0 1").unwrap();
        let board = game.get_board();

        let moves = white_king.possible_moves((3, 4), board, None);
        assert_eq!(8, moves.len());

        [
            (2, 5),
            (3, 5),
            (4, 5),
            (2, 4),
            (4, 4),
            (2, 3),
            (3, 3),
            (4, 3),
        ]
        .map(|(new_col, new_row)| {
            let expected_move = Move {
                original_position: (3, 4),
                new_position: (new_col, new_row),
                piece: ChessPiece::new(King, White),
                taken_piece: None,
                promotion: None,
            };
            assert!(moves.contains(&expected_move));
        });
    }

    #[test]
    fn king_respects_board_boundaries() {
        let white_king = ChessPiece::new(King, Color::White);
        let game = build_game_from_string("8/8/8/8/8/8/8/K7 w - - 0 1").unwrap();
        let board = game.get_board();

        let moves = white_king.possible_moves((0, 0), board, None);
        assert_eq!(3, moves.len());

        [(0, 1), (1, 1), (1, 0)].map(|(new_col, new_row)| {
            let expected_move = Move {
                original_position: (0, 0),
                new_position: (new_col, new_row),
                piece: ChessPiece::new(King, White),
                taken_piece: None,
                promotion: None,
            };
            assert!(moves.contains(&expected_move));
        });
    }

    #[test]
    fn king_respects_teammates() {
        let white_king = ChessPiece::new(King, Color::White);
        let game = build_game_from_string("8/8/2PPP3/2PKP3/2PPP3/8/8/8 w - - 0 1").unwrap();
        let board = game.get_board();

        let moves = white_king.possible_moves((3, 4), board, None);
        assert_eq!(0, moves.len());
    }

    #[test]
    fn king_can_take_opponents() {
        let white_king = ChessPiece::new(King, Color::White);
        let game = build_game_from_string("8/8/2ppp3/2pKp3/2ppp3/8/8/8 w - - 0 1").unwrap();
        let board = game.get_board();

        let moves = white_king.possible_moves((3, 4), board, None);
        assert_eq!(8, moves.len());

        [
            (2, 5),
            (3, 5),
            (4, 5),
            (2, 4),
            (4, 4),
            (2, 3),
            (3, 3),
            (4, 3),
        ]
        .map(|(new_col, new_row)| {
            let expected_move = Move {
                original_position: (3, 4),
                new_position: (new_col, new_row),
                piece: ChessPiece::new(King, White),
                taken_piece: Some(ChessPiece::new(Pawn, Black)),
                promotion: None,
            };
            assert!(moves.contains(&expected_move));
        });
    }
}
