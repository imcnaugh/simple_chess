use crate::chess_move::ChessMoveType;
use crate::piece::{ChessPiece, PieceType};
use crate::Color;
use game_board::Board;

pub fn as_utf_str(color: Color) -> &'static str {
    match color {
        Color::White => "♗",
        Color::Black => "♝",
    }
}

pub fn as_fen_char(color: Color) -> char {
    match color {
        Color::White => 'B',
        Color::Black => 'b',
    }
}

pub fn possible_moves(
    color: Color,
    position: (usize, usize),
    board: &Board<ChessPiece>,
) -> Vec<ChessMoveType> {
    let mut possible_moves: Vec<ChessMoveType> = Vec::new();

    let directions = [(1i32, 1), (1, -1), (-1, 1), (-1, -1)];

    for dir in directions.iter() {
        let mut x = position.0 as i32 + dir.0;
        let mut y = position.1 as i32 + dir.1;
        while x >= 0 && y >= 0 && x < board.get_width() as i32 && y < board.get_height() as i32 {
            if let Some(piece) = board.get_piece_at_space(x as usize, y as usize) {
                if piece.get_color() != color {
                    possible_moves.push(ChessMoveType::Move {
                        original_position: position,
                        new_position: (x as usize, y as usize),
                        piece: ChessPiece::new(PieceType::Bishop, color),
                        taken_piece: Some(*piece),
                        promotion: None,
                    });
                }
            } else {
                possible_moves.push(ChessMoveType::Move {
                    original_position: position,
                    new_position: (x as usize, y as usize),
                    piece: ChessPiece::new(PieceType::Bishop, color),
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::codec::forsyth_edwards_notation::build_game_from_string;
    use crate::ChessMoveType::Move;

    #[test]
    fn bishop_movement() {
        let white_bishop = ChessPiece {
            piece_type: PieceType::Bishop,
            color: Color::White,
        };
        let game = build_game_from_string("8/8/2B5/8/8/8/8/8 w KQkq - 0 1").unwrap();
        let board = game.get_board();

        let moves = white_bishop.possible_moves((2, 2), board, None);
        assert_eq!(moves.len(), 11);

        let expected_new_positions = [
            (0, 0),
            (0, 4),
            (1, 1),
            (1, 3),
            (3, 1),
            (3, 3),
            (4, 0),
            (4, 4),
            (5, 5),
            (6, 6),
            (7, 7),
        ];

        let expected_moves = expected_new_positions.map(|(new_col, new_row)| -> ChessMoveType {
            Move {
                original_position: (2, 2),
                new_position: (new_col, new_row),
                piece: ChessPiece::new(PieceType::Bishop, Color::White),
                taken_piece: None,
                promotion: None,
            }
        });

        for expected_move in expected_moves {
            assert!(moves.contains(&expected_move));
        }
    }
}
