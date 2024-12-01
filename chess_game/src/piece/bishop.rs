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
                    break;
                } else {
                    break;
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
    use crate::piece::PieceType::{King, Pawn, Queen};
    use crate::ChessMoveType::Move;
    use crate::Color::{Black, White};

    #[test]
    fn bishop_movement() {
        let white_bishop = ChessPiece {
            piece_type: PieceType::Bishop,
            color: Color::White,
        };
        let game = build_game_from_string("8/8/2B5/8/8/8/8/8 w KQkq - 0 1").unwrap();
        let board = game.get_board();

        let moves = white_bishop.possible_moves((2, 5), board, None);
        assert_eq!(moves.len(), 11);

        [
            (0, 7),
            (0, 3),
            (1, 6),
            (1, 4),
            (3, 6),
            (3, 4),
            (4, 7),
            (4, 3),
            (5, 2),
            (6, 1),
            (7, 0),
        ]
        .map(|(new_col, new_row)| {
            let expected_move = Move {
                original_position: (2, 5),
                new_position: (new_col, new_row),
                piece: ChessPiece::new(PieceType::Bishop, Color::White),
                taken_piece: None,
                promotion: None,
            };
            assert!(moves.contains(&expected_move));
        });
    }

    #[test]
    fn bishop_blocked_by_friendly_pieces_movement() {
        let white_bishop = ChessPiece {
            piece_type: PieceType::Bishop,
            color: Color::White,
        };
        let game = build_game_from_string("8/8/2B5/3K4/8/8/8/8 w KQkq - 0 1").unwrap();
        let board = game.get_board();

        let moves = white_bishop.possible_moves((2, 5), board, None);
        assert_eq!(6, moves.len());

        [(0, 7), (0, 3), (1, 6), (1, 4), (3, 6), (4, 7)].map(|(new_col, new_row)| {
            let expected_move = Move {
                original_position: (2, 5),
                new_position: (new_col, new_row),
                piece: ChessPiece::new(PieceType::Bishop, Color::White),
                taken_piece: None,
                promotion: None,
            };
            assert!(moves.contains(&expected_move));
        });
    }

    #[test]
    fn bishop_can_take() {
        let black_bishop = ChessPiece {
            piece_type: PieceType::Bishop,
            color: Black,
        };
        let game = build_game_from_string("8/1P1Q4/2b5/3K4/8/8/8/8 w KQkq - 0 1").unwrap();
        let board = game.get_board();

        let moves = black_bishop.possible_moves((2, 5), board, None);

        [
            (0, 3, None),
            (1, 4, None),
            (1, 6, Some(Pawn)),
            (3, 4, Some(King)),
            (3, 6, Some(Queen)),
        ]
        .map(|(new_col, new_row, take)| {
            let taken_piece = match take {
                None => None,
                Some(t) => Some(ChessPiece::new(t, White)),
            };
            let expected_move = Move {
                original_position: (2, 5),
                new_position: (new_col, new_row),
                piece: ChessPiece::new(PieceType::Bishop, Black),
                taken_piece,
                promotion: None,
            };
            assert!(moves.contains(&expected_move));
        });
    }
}
