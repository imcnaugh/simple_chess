use crate::chess_move::ChessMoveType;
use crate::piece::{ChessPiece, PieceType};
use crate::Color;
use game_board::Board;

pub fn as_utf_str(color: Color) -> &'static str {
    match color {
        Color::White => "♖",
        Color::Black => "♜",
    }
}

pub fn as_fen_char(color: Color) -> char {
    match color {
        Color::White => 'R',
        Color::Black => 'r',
    }
}

pub fn as_pgn_char() -> Option<char> {
    Some('R')
}

pub fn possible_moves(
    color: Color,
    position: (usize, usize),
    board: &Board<ChessPiece>,
) -> Vec<ChessMoveType> {
    let mut possible_moves: Vec<ChessMoveType> = Vec::new();

    let directions = [(0i32, 1), (0, -1), (1, 0), (-1, 0)];
    for dir in directions.iter() {
        let mut x = position.0 as i32 + dir.0;
        let mut y = position.1 as i32 + dir.1;
        while x >= 0 && y >= 0 && x < board.get_width() as i32 && y < board.get_height() as i32 {
            if let Some(piece) = board.get_piece_at_space(x as usize, y as usize) {
                if piece.get_color() != color {
                    possible_moves.push(ChessMoveType::Move {
                        original_position: position,
                        new_position: (x as usize, y as usize),
                        piece: ChessPiece::new(PieceType::Rook, color),
                        taken_piece: Some(*piece),
                        promotion: None,
                    });
                }
                break;
            } else {
                possible_moves.push(ChessMoveType::Move {
                    original_position: position,
                    new_position: (x as usize, y as usize),
                    piece: ChessPiece::new(PieceType::Rook, color),
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
    use crate::piece::PieceType::{Bishop, Knight, Pawn, Rook};
    use crate::ChessMoveType::Move;
    use crate::Color::{Black, White};

    #[test]
    fn rook_can_move_freely() {
        let black_rook = ChessPiece::new(Rook, Black);
        let game = build_game_from_string("8/8/6r1/8/8/8/8/8 w - - 0 1").unwrap();
        let board = game.get_board();

        let moves = black_rook.possible_moves((6, 5), board, None);
        assert_eq!(14, moves.len());

        [
            (6, 6),
            (6, 7),
            (7, 5),
            (6, 4),
            (6, 3),
            (6, 2),
            (6, 1),
            (6, 0),
            (5, 5),
            (4, 5),
            (3, 5),
            (2, 5),
            (1, 5),
            (0, 5),
        ]
        .map(|new_position| {
            let expected_move = Move {
                original_position: (6, 5),
                new_position,
                piece: ChessPiece::new(Rook, Black),
                taken_piece: None,
                promotion: None,
            };
            assert!(moves.contains(&expected_move));
        });
    }

    #[test]
    fn rook_is_blocked_by_teammates() {
        let black_rook = ChessPiece::new(Rook, Black);
        let game = build_game_from_string("6n1/8/3q2rp/8/8/8/6b1/8 w - - 0 1").unwrap();
        let board = game.get_board();

        let moves = black_rook.possible_moves((6, 5), board, None);
        assert_eq!(6, moves.len());

        [(6, 6), (6, 4), (6, 3), (6, 2), (5, 5), (4, 5)].map(|new_position| {
            let expected_move = Move {
                original_position: (6, 5),
                new_position,
                piece: ChessPiece::new(Rook, Black),
                taken_piece: None,
                promotion: None,
            };
            assert!(moves.contains(&expected_move));
        });
    }

    #[test]
    fn rook_can_take_opponent() {
        let black_rook = ChessPiece::new(Rook, Black);
        let game = build_game_from_string("6N1/8/3P2rP/8/8/8/6B1/8 w - - 0 1").unwrap();
        let board = game.get_board();

        let moves = black_rook.possible_moves((6, 5), board, None);
        assert_eq!(10, moves.len());

        [
            ((6, 6), None),
            ((6, 4), None),
            ((6, 3), None),
            ((6, 2), None),
            ((5, 5), None),
            ((4, 5), None),
            ((3, 5), Some(Pawn)),
            ((6, 7), Some(Knight)),
            ((7, 5), Some(Pawn)),
            ((6, 1), Some(Bishop)),
        ]
        .map(|(new_position, taken_piece)| {
            let taken_piece = match taken_piece {
                Some(piece) => Some(ChessPiece::new(piece, White)),
                None => None,
            };
            let expected_move = Move {
                original_position: (6, 5),
                new_position,
                piece: ChessPiece::new(Rook, Black),
                taken_piece,
                promotion: None,
            };
            assert!(moves.contains(&expected_move));
        });
    }
}
