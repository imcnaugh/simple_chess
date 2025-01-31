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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::codec::forsyth_edwards_notation::build_game_from_string;
    use crate::piece::PieceType::{Bishop, Knight, Pawn, Queen, Rook};
    use crate::ChessMoveType::Move;
    use crate::Color::{Black, White};

    #[test]
    fn knight_can_move_to_open_spaces() {
        let black_knight = ChessPiece::new(Knight, Black);
        let game = build_game_from_string("8/8/8/8/4n3/8/8/8 w - - 0 1").unwrap();
        let board = game.get_board();

        let moves = black_knight.possible_moves((4, 3), board, None);
        assert_eq!(8, moves.len());

        [
            (2, 2),
            (3, 1),
            (5, 1),
            (6, 2),
            (2, 4),
            (3, 5),
            (5, 5),
            (6, 4),
        ]
        .map(|(new_col, new_row)| {
            let expected_move = Move {
                original_position: (4, 3),
                new_position: (new_col, new_row),
                piece: ChessPiece::new(Knight, Black),
                taken_piece: None,
                promotion: None,
            };
            assert!(moves.contains(&expected_move));
        });
    }

    #[test]
    fn knight_respects_board_boundaries() {
        let black_knight = ChessPiece::new(Knight, Black);
        let game = build_game_from_string("8/8/8/8/8/8/8/n7 w - - 0 1").unwrap();
        let board = game.get_board();

        let moves = black_knight.possible_moves((0, 0), board, None);
        assert_eq!(2, moves.len());

        [(1, 2), (2, 1)].map(|(new_col, new_row)| {
            let expected_move = Move {
                original_position: (0, 0),
                new_position: (new_col, new_row),
                piece: ChessPiece::new(Knight, Black),
                taken_piece: None,
                promotion: None,
            };
            assert!(moves.contains(&expected_move));
        });
    }

    #[test]
    fn knight_respects_teammates() {
        let black_knight = ChessPiece::new(Knight, Black);
        let game = build_game_from_string("8/8/3n1q2/2b3r1/4n3/2b3r1/3p1p2/8 w - - 0 1").unwrap();
        let board = game.get_board();

        let moves = black_knight.possible_moves((4, 3), board, None);
        assert_eq!(0, moves.len());
    }

    #[test]
    fn knight_yeets_onto_opponents() {
        let black_knight = ChessPiece::new(Knight, White);
        let game =
            build_game_from_string("8/8/3n1q2/2bPPPr1/3PNP2/2bPPPr1/3p1p2/8 w - - 0 1").unwrap();
        let board = game.get_board();

        let moves = black_knight.possible_moves((4, 3), board, None);
        assert_eq!(8, moves.len());

        [
            (2, 2, Bishop),
            (3, 1, Pawn),
            (5, 1, Pawn),
            (6, 2, Rook),
            (6, 4, Rook),
            (5, 5, Queen),
            (3, 5, Knight),
            (2, 4, Bishop),
        ]
        .map(|(new_col, new_row, taken_piece)| {
            let expected_move = Move {
                original_position: (4, 3),
                new_position: (new_col, new_row),
                piece: ChessPiece::new(Knight, White),
                taken_piece: Some(ChessPiece::new(taken_piece, Black)),
                promotion: None,
            };
            assert!(moves.contains(&expected_move));
        });
    }
}
