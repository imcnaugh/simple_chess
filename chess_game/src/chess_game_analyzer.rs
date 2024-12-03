use crate::piece::ChessPiece;
use crate::piece::PieceType::King;
use crate::{ChessGame, ChessMoveType, Color};
use game_board::Board;

pub fn get_legal_moves(mut game: ChessGame) -> Vec<ChessMoveType> {
    let current_turn = game.get_current_players_turn();
    let last_move = game.get_last_move();
    let board = game.get_board();

    let all_moves = get_all_moves_for_color(current_turn, board, last_move);
    all_moves
        .into_iter()
        .filter(|possible_move| {
            let board = game.get_board_mut();
            possible_move.make_move(board);
            let in_check = is_in_check(current_turn, &board, Some(possible_move));
            possible_move.undo_move(board);
            !in_check
        })
        .collect::<Vec<ChessMoveType>>()
}

pub fn is_in_check(color: Color, board: &Board<ChessPiece>, last_move: Option<&ChessMoveType>) -> bool {
    for row in 0..board.get_height() {
        for col in 0..board.get_width() {
            if let Some(piece) = board.get_piece_at_space(col, row) {
                if piece.get_color() == color.opposite() {
                    let moves = piece.possible_moves((col, row), board, last_move);
                    for m in moves {
                        match m {
                            ChessMoveType::Move { taken_piece, .. } => {
                                if let Some(taken_piece) = taken_piece {
                                    if taken_piece.get_piece_type() == King {
                                        return true;
                                    }
                                }
                            }
                            ChessMoveType::EnPassant { taken_piece, .. } => {
                                if taken_piece.get_piece_type() == King {
                                    return true;
                                }
                            }
                            ChessMoveType::Castle {..} => return false,
                        }
                    }
                }
            }
        }
    }
    false
}

fn get_all_moves_for_color(
    color: Color,
    board: &Board<ChessPiece>,
    last_move: Option<&ChessMoveType>,
) -> Vec<ChessMoveType> {
    let mut moves: Vec<ChessMoveType> = Vec::new();

    for row in 0..board.get_height() {
        for col in 0..board.get_width() {
            if let Some(piece) = board.get_piece_at_space(col, row) {
                if piece.get_color() == color {
                    moves.append(&mut piece.possible_moves((col, row), board, last_move));
                }
            }
        }
    }

    moves
}

#[cfg(test)]
mod tests {
    use crate::ChessMoveType::Move;
    use crate::codec::forsyth_edwards_notation::build_game_from_string;
    use crate::Color::{Black, White};
    use crate::piece::PieceType::{Bishop, Knight, Pawn, Queen, Rook};
    use super::*;

    #[test]
    fn get_legal_moves_for_starting_position() {
        let game = ChessGame::new();
        let legal_moves = get_legal_moves(game);
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
        let game = build_game_from_string("k6R/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b - - 0 1").unwrap();
        let legal_moves = get_legal_moves(game);
        assert_eq!(legal_moves.len(), 0);
    }

    #[test]
    fn king_in_check_limits_legal_moves() {
        let game = build_game_from_string("k6R/1ppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b - - 0 1").unwrap();
        let legal_moves = get_legal_moves(game);
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
        let game = build_game_from_string("8/8/8/8/2n5/1p5r/K7/BB6 w - - 0 1").unwrap();
        let legal_moves = get_legal_moves(game);
        assert_eq!(0, legal_moves.len());
    }

    #[test]
    fn pieces_can_be_pinned_to_the_king() {
        let game = build_game_from_string("K2B3r/8/8/8/8/8/8/8 w - - 0 1").unwrap();
        let legal_moves = get_legal_moves(game);
        assert_eq!(3, legal_moves.len());
        let make_king_moves = |new_position| -> ChessMoveType {
            Move {
                original_position: (0,7),
                new_position,
                piece: ChessPiece::new(King, White),
                taken_piece: None,
                promotion: None,
            }
        };
        [(1,7), (1,6), (0,6)].iter().for_each(|new_position| {
            let m = make_king_moves(*new_position);
            assert!(legal_moves.contains(&m));
        })
    }

    #[test]
    fn pawns_can_promote() {
        let game = build_game_from_string("8/4P3/8/8/8/8/8/8 w - - 0 1").unwrap();
        let legal_moves = get_legal_moves(game);
        assert_eq!(4, legal_moves.len());
        [Queen, Rook, Bishop, Knight].iter().for_each(|promotion_option| {
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
        let game = build_game_from_string("2r5/8/8/2Pp4/8/8/8/2K5 w - d6 0 1").unwrap();
        let legal_moves = get_legal_moves(game);
        assert_eq!(6, legal_moves.len());
    }

    #[test]
    fn can_en_passant() {
        let game = build_game_from_string("8/8/8/2Pp4/8/8/8/2K5 w - d6 0 1").unwrap();
        let legal_moves = get_legal_moves(game);
        assert_eq!(7, legal_moves.len());
    }

    #[test]
    fn stalemate_should_yield_no_legal_moves() {
        let game = build_game_from_string("1r4b1/8/8/8/8/8/8/K7 w - - 0 1").unwrap();
        let legal_moves = get_legal_moves(game);
        assert_eq!(0, legal_moves.len());
    }
}