use crate::chess_game_state_analyzer::is_in_check;
use crate::piece::ChessPiece;
use crate::piece::PieceType::King;
use crate::ChessMoveType::{Castle, Move};
use crate::{ChessGame, ChessMoveType, Color};

///
/// Returns a vector of legal moves for the current player's turn in the given simple_chess game.
///
/// # Arguments
///
/// * `game` - A mutable reference to the `ChessGame` instance for which legal moves need to be determined.
///
/// # Returns
///
/// A vector of `ChessMoveType` that represents all possible legal moves that the current
/// player can make without putting their king in check.
pub fn get_legal_moves(game: &mut ChessGame) -> Vec<ChessMoveType> {
    let current_turn = game.get_current_players_turn();

    get_all_moves_for_color(current_turn, game)
        .into_iter()
        .filter(|possible_move| {
            let board = game.get_board_mut();
            possible_move.make_move(board);
            let in_check = is_in_check(current_turn, board);
            possible_move.undo_move(board);
            !in_check
        })
        .collect::<Vec<ChessMoveType>>()
}

fn get_all_moves_for_color(color: Color, game: &mut ChessGame) -> Vec<ChessMoveType> {
    let mut moves: Vec<ChessMoveType> = Vec::new();
    let board = game.get_board();

    for row in 0..board.get_height() {
        for col in 0..board.get_width() {
            if let Some(piece) = board.get_piece_at_space(col, row) {
                if piece.get_color() == color {
                    moves.append(&mut piece.possible_moves(
                        (col, row),
                        board,
                        game.get_last_move(),
                    ));
                }
            }
        }
    }

    let castling_moves = generate_possible_castling_moves(color, game);
    moves.extend(castling_moves);

    moves
}

fn generate_possible_castling_moves(color: Color, game: &mut ChessGame) -> Vec<ChessMoveType> {
    let castling_rights = game.get_castling_rights();
    let (long_castle, short_castle) = match color {
        Color::White => (castling_rights.0, castling_rights.1),
        Color::Black => (castling_rights.2, castling_rights.3),
    };

    let mut moves = Vec::new();
    let board = game.get_board_mut();
    let row = match color {
        Color::White => 0,
        Color::Black => board.get_height() - 1,
    };

    if is_in_check(color, board) {
        return moves;
    }
    if long_castle {
        for col in 1..board.get_width() - 1 {
            if let Some(piece) = board.get_piece_at_space(col, row) {
                if piece.get_piece_type() != King || piece.get_color() != color {
                    break;
                }
                let tmp_move = Move {
                    original_position: (col, row),
                    new_position: (col - 1, row),
                    piece: ChessPiece::new(King, color),
                    taken_piece: None,
                    promotion: None,
                };
                tmp_move.make_move(board);
                if !is_in_check(color, board) {
                    moves.push(Castle {
                        rook_original_position: (0, row),
                        rook_new_position: (col - 1, row),
                        king_original_position: (col, row),
                        king_new_position: (col - 2, row),
                    })
                }
                tmp_move.undo_move(board);
            }
        }
    }
    if short_castle {
        for col in (0..board.get_width() - 1).rev() {
            if let Some(piece) = board.get_piece_at_space(col, row) {
                if piece.get_piece_type() != King || piece.get_color() != color {
                    break;
                }
                let tmp_move = Move {
                    original_position: (col, row),
                    new_position: (col + 1, row),
                    piece: ChessPiece::new(King, color),
                    taken_piece: None,
                    promotion: None,
                };
                tmp_move.make_move(board);
                if !is_in_check(color, board) {
                    moves.push(Castle {
                        rook_original_position: (board.get_width() - 1, row),
                        rook_new_position: (col + 1, row),
                        king_original_position: (col, row),
                        king_new_position: (col + 2, row),
                    });
                }
                tmp_move.undo_move(board);
            }
        }
    }

    moves
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::codec::forsyth_edwards_notation::build_game_from_string;
    use crate::piece::PieceType::{Bishop, Knight, Pawn, Queen, Rook};
    use crate::ChessMoveType::Move;
    use crate::Color::{Black, White};

    #[test]
    fn get_legal_moves_for_starting_position() {
        let mut game = ChessGame::new();
        let legal_moves = get_legal_moves(&mut game);
        assert_eq!(20, legal_moves.len());

        for col in 0..8 {
            for pawn_move_length in 1..=2 {
                let move_type = Move {
                    original_position: (col, 1),
                    new_position: (col, pawn_move_length + 1),
                    piece: ChessPiece::new(Pawn, White),
                    taken_piece: None,
                    promotion: None,
                };
                assert!(legal_moves.contains(&move_type));
            }
        }

        let knight_a3 = Move {
            original_position: (1, 0),
            new_position: (0, 2),
            piece: ChessPiece::new(Knight, White),
            taken_piece: None,
            promotion: None,
        };
        let knight_c3 = Move {
            original_position: (1, 0),
            new_position: (2, 2),
            piece: ChessPiece::new(Knight, White),
            taken_piece: None,
            promotion: None,
        };
        let knight_f3 = Move {
            original_position: (6, 0),
            new_position: (5, 2),
            piece: ChessPiece::new(Knight, White),
            taken_piece: None,
            promotion: None,
        };
        let knight_h3 = Move {
            original_position: (6, 0),
            new_position: (7, 2),
            piece: ChessPiece::new(Knight, White),
            taken_piece: None,
            promotion: None,
        };
        assert!(legal_moves.contains(&knight_a3));
        assert!(legal_moves.contains(&knight_c3));
        assert!(legal_moves.contains(&knight_f3));
        assert!(legal_moves.contains(&knight_h3));
    }

    #[test]
    fn player_in_checkmate_has_no_legal_moves() {
        let mut game =
            build_game_from_string("k6R/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b - - 0 1").unwrap();
        let legal_moves = get_legal_moves(&mut game);
        assert_eq!(legal_moves.len(), 0);
    }

    #[test]
    fn king_in_check_limits_legal_moves() {
        let mut game =
            build_game_from_string("k6R/1ppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b - - 0 1").unwrap();
        let legal_moves = get_legal_moves(&mut game);
        assert_eq!(1, legal_moves.len());
        let expected_move = Move {
            original_position: (0, 7),
            new_position: (0, 6),
            piece: ChessPiece::new(King, Black),
            taken_piece: None,
            promotion: None,
        };
        assert_eq!(expected_move, legal_moves[0])
    }

    #[test]
    fn more_complex_checkmate() {
        let mut game = build_game_from_string("8/8/8/8/2n5/1p5r/K7/BB6 w - - 0 1").unwrap();
        let legal_moves = get_legal_moves(&mut game);
        assert_eq!(0, legal_moves.len());
    }

    #[test]
    fn pieces_can_be_pinned_to_the_king() {
        let mut game = build_game_from_string("K2B3r/8/8/8/8/8/8/8 w - - 0 1").unwrap();
        let legal_moves = get_legal_moves(&mut game);
        assert_eq!(3, legal_moves.len());
        let make_king_moves = |new_position| -> ChessMoveType {
            Move {
                original_position: (0, 7),
                new_position,
                piece: ChessPiece::new(King, White),
                taken_piece: None,
                promotion: None,
            }
        };
        [(1, 7), (1, 6), (0, 6)].iter().for_each(|new_position| {
            let m = make_king_moves(*new_position);
            assert!(legal_moves.contains(&m));
        })
    }

    #[test]
    fn pawns_can_promote() {
        let mut game = build_game_from_string("8/4P3/8/8/8/8/8/8 w - - 0 1").unwrap();
        let legal_moves = get_legal_moves(&mut game);
        assert_eq!(4, legal_moves.len());
        [Queen, Rook, Bishop, Knight]
            .iter()
            .for_each(|promotion_option| {
                let expected_move = Move {
                    original_position: (4, 6),
                    new_position: (4, 7),
                    piece: ChessPiece::new(Pawn, White),
                    taken_piece: None,
                    promotion: Some(ChessPiece::new(*promotion_option, White)),
                };
                assert!(legal_moves.contains(&expected_move));
            })
    }

    #[test]
    fn cannot_en_passant_if_pawn_pinned() {
        let mut game = build_game_from_string("2r5/8/8/2Pp4/8/8/8/2K5 w - d6 0 1").unwrap();
        let legal_moves = get_legal_moves(&mut game);
        assert_eq!(6, legal_moves.len());
    }

    #[test]
    fn can_en_passant() {
        let mut game = build_game_from_string("8/8/8/2Pp4/8/8/8/2K5 w - d6 0 1").unwrap();
        let legal_moves = get_legal_moves(&mut game);
        assert_eq!(7, legal_moves.len());
    }

    #[test]
    fn stalemate_should_yield_no_legal_moves() {
        let mut game = build_game_from_string("1r4b1/8/8/8/8/8/8/K7 w - - 0 1").unwrap();
        let legal_moves = get_legal_moves(&mut game);
        assert_eq!(0, legal_moves.len());
    }

    #[test]
    fn can_castle_if_in_check() {
        let mut game = build_game_from_string("8/8/8/8/8/8/8/R3K2R w KQkq - 0 1").unwrap();
        let legal_moves = get_legal_moves(&mut game);

        assert!(legal_moves.contains(&Castle {
            rook_original_position: (7, 0),
            rook_new_position: (5, 0),
            king_original_position: (4, 0),
            king_new_position: (6, 0),
        }));
        assert!(legal_moves.contains(&Castle {
            rook_original_position: (0, 0),
            rook_new_position: (3, 0),
            king_original_position: (4, 0),
            king_new_position: (2, 0),
        }))
    }

    #[test]
    fn can_not_castle_if_in_check() {
        let mut game = build_game_from_string("4r3/8/8/8/8/8/8/R3K2R w KQkq - 0 1").unwrap();
        let legal_moves = get_legal_moves(&mut game);
        assert_eq!(4, legal_moves.len());
        for mov in legal_moves {
            if let Castle { .. } = mov {
                panic!("Should not be able to castle");
            }
        }
    }

    #[test]
    fn can_not_castle_through_check() {
        let mut game = build_game_from_string("3r4/8/8/8/8/8/8/R3K3 w Q - 0 1").unwrap();
        let legal_moves = get_legal_moves(&mut game);
        for mov in legal_moves {
            if let Castle { .. } = mov {
                panic!("Should not be able to castle");
            }
        }
    }

    #[test]
    fn can_not_castle_into_check() {
        let mut game = build_game_from_string("2r5/8/8/8/8/8/8/R3K3 w Q - 0 1").unwrap();
        let legal_moves = get_legal_moves(&mut game);
        for mov in legal_moves {
            if let Castle { .. } = mov {
                panic!("Should not be able to castle");
            }
        }
    }

    #[test]
    fn can_not_castle_through_pieces() {
        let mut game = build_game_from_string("8/8/8/8/8/8/8/R1P1K1bR w KQ - 0 1").unwrap();
        let legal_moves = get_legal_moves(&mut game);
        for mov in legal_moves {
            if let Castle { .. } = mov {
                panic!("Should not be able to castle");
            }
        }
    }

    #[test]
    fn can_not_castle_if_you_do_not_have_the_right() {
        let mut game = build_game_from_string("8/8/8/8/8/8/8/R3K2R w kq - 0 1").unwrap();
        let legal_moves = get_legal_moves(&mut game);
        for mov in legal_moves {
            if let Castle { .. } = mov {
                panic!("Should not be able to castle");
            }
        }
    }
}
