use crate::chess_move::ChessMoveType;
use crate::piece::{ChessPiece, PieceType};
use crate::Color;
use game_board::Board;

const PROMOTION_OPTIONS: [PieceType; 4] = [
    PieceType::Queen,
    PieceType::Rook,
    PieceType::Bishop,
    PieceType::Knight,
];

pub fn as_utf_str(color: Color) -> &'static str {
    match color {
        Color::White => "♙",
        Color::Black => "♟",
    }
}

pub fn possible_moves(
    color: Color,
    position: (usize, usize),
    board: &Board<ChessPiece>,
    last_move_type: Option<&ChessMoveType>,
) -> Vec<ChessMoveType> {
    let mut possible_moves: Vec<ChessMoveType> = Vec::new();

    let forward_direction = match color {
        Color::White => 1,
        Color::Black => -1,
    };

    let promotion_row = match color {
        Color::White => board.get_height() - 1,
        Color::Black => 0,
    };

    let next_row = position.1 as i32 + forward_direction;
    if next_row < 0 || next_row > board.get_height() as i32 {
        return possible_moves;
    }

    // Simple move forward
    if board
        .get_piece_at_space(position.0, next_row as usize)
        .is_none()
    {
        possible_moves.append(&mut create_possible_moves(
            position,
            (position.0, next_row as usize),
            color,
            None,
            next_row as usize == promotion_row,
        ));

        // moving 2 spaces from starting row
        let starting_row = match color {
            Color::White => 1,
            Color::Black => board.get_height() - 2,
        };
        if position.1 == starting_row {
            let double_next_row = position.1 as i32 + 2 * forward_direction;
            if double_next_row >= 0 && double_next_row < board.get_height() as i32 {
                if board
                    .get_piece_at_space(position.0, double_next_row as usize)
                    .is_none()
                {
                    possible_moves.append(&mut create_possible_moves(
                        position,
                        (position.0, double_next_row as usize),
                        color,
                        None,
                        next_row as usize == promotion_row,
                    ));
                }
            }
        }
    }

    // Taking to the left
    if position.0 > 0 {
        if let Some(piece) = board.get_piece_at_space(position.0 - 1, next_row as usize) {
            if piece.color != color {
                possible_moves.append(&mut create_possible_moves(
                    position,
                    (position.0 - 1, next_row as usize),
                    color,
                    Some(*piece),
                    next_row as usize == promotion_row,
                ));
            }
        }

        // En Passant
        if let Some(ChessMoveType::Move {
            piece,
            new_position,
            original_position,
            ..
        }) = last_move_type
        {
            if piece.piece_type == PieceType::Pawn && piece.color != color {
                let rows_moved = if original_position.1 < new_position.1 {
                    new_position.1 - original_position.1
                } else {
                    original_position.1 - new_position.1
                };

                if rows_moved == 2
                    && new_position.0 == position.0 - 1
                    && new_position.1 == position.1
                {
                    possible_moves.append(&mut create_possible_en_passant(
                        position,
                        (position.0 - 1, next_row as usize),
                        color,
                        *piece,
                        *new_position,
                        next_row as usize == promotion_row,
                    ));
                }
            }
        }
    }

    // Taking to the right
    if position.0 < board.get_width() - 1 {
        if let Some(piece) = board.get_piece_at_space(position.0 + 1, next_row as usize) {
            if piece.color != color {
                possible_moves.append(&mut create_possible_moves(
                    position,
                    (position.0 + 1, next_row as usize),
                    color,
                    Some(*piece),
                    next_row as usize == promotion_row,
                ));
            }
        }

        // En Passant
        if let Some(ChessMoveType::Move {
            piece,
            new_position,
            original_position,
            ..
        }) = last_move_type
        {
            if piece.piece_type == PieceType::Pawn && piece.color != color {
                let rows_moved = if original_position.1 < new_position.1 {
                    new_position.1 - original_position.1
                } else {
                    original_position.1 - new_position.1
                };

                if rows_moved == 2
                    && new_position.0 == position.0 + 1
                    && new_position.1 == position.1
                {
                    possible_moves.append(&mut create_possible_en_passant(
                        position,
                        (position.0 + 1, next_row as usize),
                        color,
                        *piece,
                        *new_position,
                        next_row as usize == promotion_row,
                    ));
                }
            }
        }
    }

    possible_moves
}

fn create_possible_moves(
    original_position: (usize, usize),
    new_position: (usize, usize),
    color: Color,
    taken_piece: Option<ChessPiece>,
    can_promote: bool,
) -> Vec<ChessMoveType> {
    let mut possible_moves: Vec<ChessMoveType> = Vec::new();
    if can_promote {
        for option in PROMOTION_OPTIONS.iter() {
            possible_moves.push(ChessMoveType::Move {
                original_position,
                new_position,
                piece: ChessPiece::new(PieceType::Pawn, color),
                taken_piece,
                promotion: Some(ChessPiece::new(*option, color)),
            })
        }
    } else {
        possible_moves.push(ChessMoveType::Move {
            original_position,
            new_position,
            piece: ChessPiece::new(PieceType::Pawn, color),
            taken_piece,
            promotion: None,
        })
    }

    possible_moves
}

fn create_possible_en_passant(
    original_position: (usize, usize),
    new_position: (usize, usize),
    color: Color,
    taken_piece: ChessPiece,
    taken_piece_position: (usize, usize),
    can_promote: bool,
) -> Vec<ChessMoveType> {
    let mut possible_en_passants: Vec<ChessMoveType> = Vec::new();

    if can_promote {
        for option in PROMOTION_OPTIONS.iter() {
            possible_en_passants.push(ChessMoveType::EnPassant {
                original_position,
                new_position,
                piece: ChessPiece::new(PieceType::Pawn, color),
                taken_piece,
                taken_piece_position,
                promotion: Some(ChessPiece::new(*option, color)),
            })
        }
    } else {
        possible_en_passants.push(ChessMoveType::EnPassant {
            original_position,
            new_position,
            piece: ChessPiece::new(PieceType::Pawn, color),
            taken_piece,
            taken_piece_position,
            promotion: None,
        })
    }

    possible_en_passants
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::codec::forsyth_edwards_notation::build_game_from_string;
    use crate::piece::PieceType::{Bishop, King, Knight, Pawn, Queen, Rook};
    use crate::ChessMoveType::{EnPassant, Move};
    use crate::Color::{Black, White};

    #[test]
    fn white_pawn_moves_forward() {
        let white_pawn = ChessPiece::new(Pawn, White);
        let game = build_game_from_string("8/8/8/8/4P3/8/8/8 w - - 0 1").unwrap();
        let board = game.get_board();

        let moves = white_pawn.possible_moves((4, 3), board, None);
        assert_eq!(1, moves.len());

        let expected_move = Move {
            original_position: (4, 3),
            new_position: (4, 4),
            piece: ChessPiece::new(Pawn, White),
            taken_piece: None,
            promotion: None,
        };

        assert_eq!(expected_move, moves[0]);
    }

    #[test]
    fn black_pawn_moves_forward() {
        let black_pawn = ChessPiece::new(Pawn, Black);
        let game = build_game_from_string("8/8/8/8/4b3/8/8/8 w - - 0 1").unwrap();
        let board = game.get_board();

        let moves = black_pawn.possible_moves((4, 3), board, None);
        assert_eq!(1, moves.len());

        let expected_move = Move {
            original_position: (4, 3),
            new_position: (4, 2),
            piece: ChessPiece::new(Pawn, Black),
            taken_piece: None,
            promotion: None,
        };

        assert_eq!(expected_move, moves[0]);
    }

    #[test]
    fn pawn_can_not_trample_teammate() {
        let black_pawn = ChessPiece::new(Pawn, Black);
        let game = build_game_from_string("8/8/8/8/4b3/4k3/8/8 w - - 0 1").unwrap();
        let board = game.get_board();

        let moves = black_pawn.possible_moves((4, 3), board, None);
        assert_eq!(0, moves.len());
    }

    #[test]
    fn white_pawn_can_move_2_spaces_if_on_starting_square() {
        let white_pawn = ChessPiece::new(Pawn, White);
        let game = build_game_from_string("8/8/8/8/8/8/4P3/8 w - - 0 1").unwrap();
        let board = game.get_board();

        let moves = white_pawn.possible_moves((4, 1), board, None);
        assert_eq!(2, moves.len());

        [(4, 2), (4, 3)].map(|(new_col, new_row)| {
            let expected_move = Move {
                original_position: (4, 1),
                new_position: (new_col, new_row),
                piece: ChessPiece::new(Pawn, White),
                taken_piece: None,
                promotion: None,
            };

            assert!(moves.contains(&expected_move));
        });
    }

    #[test]
    fn black_pawn_can_move_2_spaces_if_on_starting_square() {
        let black_pawn = ChessPiece::new(Pawn, Black);
        let game = build_game_from_string("8/2P5/8/8/8/8/8/8 w - - 0 1").unwrap();
        let board = game.get_board();

        let moves = black_pawn.possible_moves((2, 6), board, None);
        assert_eq!(2, moves.len());

        [(2, 5), (2, 4)].map(|(new_col, new_row)| {
            let expected_move = Move {
                original_position: (2, 6),
                new_position: (new_col, new_row),
                piece: ChessPiece::new(Pawn, Black),
                taken_piece: None,
                promotion: None,
            };

            assert!(moves.contains(&expected_move));
        });
    }

    #[test]
    fn white_pawn_blocked_from_moving_2_spaces_if_on_starting_square() {
        let white_pawn = ChessPiece::new(Pawn, White);
        let game = build_game_from_string("8/8/8/8/4P3/8/4P3/8 w - - 0 1").unwrap();
        let board = game.get_board();

        let moves = white_pawn.possible_moves((4, 1), board, None);
        assert_eq!(1, moves.len());

        [(4, 2)].map(|(new_col, new_row)| {
            let expected_move = Move {
                original_position: (4, 1),
                new_position: (new_col, new_row),
                piece: ChessPiece::new(Pawn, White),
                taken_piece: None,
                promotion: None,
            };

            assert!(moves.contains(&expected_move));
        });
    }

    #[test]
    fn black_pawn_blocked_from_moving_2_spaces_if_on_starting_square() {
        let black_pawn = ChessPiece::new(Pawn, Black);
        let game = build_game_from_string("8/2P5/8/2P5/8/8/8/8 w - - 0 1").unwrap();
        let board = game.get_board();

        let moves = black_pawn.possible_moves((2, 6), board, None);
        assert_eq!(1, moves.len());

        [(2, 5)].map(|(new_col, new_row)| {
            let expected_move = Move {
                original_position: (2, 6),
                new_position: (new_col, new_row),
                piece: ChessPiece::new(Pawn, Black),
                taken_piece: None,
                promotion: None,
            };

            assert!(moves.contains(&expected_move));
        });
    }

    #[test]
    fn white_pawn_can_take_to_either_side() {
        let white_pawn = ChessPiece::new(Pawn, White);
        let game = build_game_from_string("8/8/8/2n1q3/3P4/8/8/8 w - - 0 1").unwrap();
        let board = game.get_board();

        let moves = white_pawn.possible_moves((3, 3), board, None);
        assert_eq!(3, moves.len());

        [(2, 4, Some(Knight)), (4, 4, Some(Queen)), (3, 4, None)].map(
            |(new_col, new_row, taken_piece)| {
                let taken_piece = match taken_piece {
                    Some(p) => Some(ChessPiece::new(p, Black)),
                    None => None,
                };
                let expected_move = Move {
                    original_position: (3, 3),
                    new_position: (new_col, new_row),
                    piece: ChessPiece::new(Pawn, White),
                    taken_piece,
                    promotion: None,
                };

                assert!(moves.contains(&expected_move));
            },
        );
    }

    #[test]
    fn black_pawn_can_take_to_either_side() {
        let black_pawn = ChessPiece::new(Pawn, Black);
        let game = build_game_from_string("8/8/8/3p4/2K1B3/8/8/8 w - - 0 1").unwrap();
        let board = game.get_board();

        let moves = black_pawn.possible_moves((3, 4), board, None);
        assert_eq!(3, moves.len());

        [(2, 3, Some(King)), (4, 3, Some(Bishop)), (3, 3, None)].map(
            |(new_col, new_row, taken_piece)| {
                let taken_piece = match taken_piece {
                    Some(p) => Some(ChessPiece::new(p, White)),
                    None => None,
                };
                let expected_move = Move {
                    original_position: (3, 4),
                    new_position: (new_col, new_row),
                    piece: ChessPiece::new(Pawn, Black),
                    taken_piece,
                    promotion: None,
                };

                assert!(moves.contains(&expected_move));
            },
        );
    }

    #[test]
    fn white_en_passent() {
        let white_pawn = ChessPiece::new(Pawn, White);
        let game = build_game_from_string("8/8/4N3/4Pp2/8/8/8/8 w - f6 0 1").unwrap();
        let board = game.get_board();

        let moves = white_pawn.possible_moves((4, 4), board, game.get_last_move());
        assert_eq!(1, moves.len());

        let expected_move = EnPassant {
            original_position: (4, 4),
            new_position: (5, 5),
            piece: ChessPiece::new(Pawn, White),
            taken_piece: ChessPiece::new(Pawn, Black),
            taken_piece_position: (5, 4),
            promotion: None,
        };
        assert_eq!(expected_move, moves[0]);
    }

    #[test]
    fn black_en_passent() {
        let black_pawn = ChessPiece::new(Pawn, Black);
        let game = build_game_from_string("8/8/8/8/Pp6/1n6/8/8 w - a3 0 1").unwrap();
        let board = game.get_board();

        let moves = black_pawn.possible_moves((1, 3), board, game.get_last_move());
        assert_eq!(1, moves.len());

        let expected_move = EnPassant {
            original_position: (1, 3),
            new_position: (0, 2),
            piece: ChessPiece::new(Pawn, Black),
            taken_piece: ChessPiece::new(Pawn, White),
            taken_piece_position: (0, 3),
            promotion: None,
        };
        assert_eq!(expected_move, moves[0]);
    }

    #[test]
    fn white_pawn_can_promote() {
        let white_pawn = ChessPiece::new(Pawn, White);
        let game = build_game_from_string("8/P7/8/8/8/8/8/8 w - - 0 1").unwrap();
        let board = game.get_board();

        let moves = white_pawn.possible_moves((0, 6), board, game.get_last_move());

        assert_eq!(4, moves.len());

        [Queen, Rook, Bishop, Knight].map(|promotion_option| {
            let expected_move = Move {
                original_position: (0, 6),
                new_position: (0, 7),
                piece: ChessPiece::new(Pawn, White),
                taken_piece: None,
                promotion: Some(ChessPiece::new(promotion_option, White)),
            };
            assert!(moves.contains(&expected_move));
        });
    }

    #[test]
    fn black_pawn_can_promote() {
        let black_pawn = ChessPiece::new(Pawn, Black);
        let game = build_game_from_string("8/8/8/8/8/8/6p1/8 w - - 0 1").unwrap();
        let board = game.get_board();

        let moves = black_pawn.possible_moves((6, 1), board, game.get_last_move());

        assert_eq!(4, moves.len());

        [Queen, Rook, Bishop, Knight].map(|promotion_option| {
            let expected_move = Move {
                original_position: (6, 1),
                new_position: (6, 0),
                piece: ChessPiece::new(Pawn, Black),
                taken_piece: None,
                promotion: Some(ChessPiece::new(promotion_option, Black)),
            };
            assert!(moves.contains(&expected_move));
        });
    }
}
