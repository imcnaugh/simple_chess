use crate::chess_game_state_analyzer::GameState;
use crate::codec::long_algebraic_notation::encode_move_as_long_algebraic_notation;
use crate::piece::{ChessPiece, PieceType};
use crate::{ChessGame, ChessMoveType, Color};
use game_board::{get_file_name, get_rank_name, get_square_name_from_row_and_col, Board};

fn encode_move_as_algebraic_notation(
    chess_move_type: &ChessMoveType,
    starting_position: &Board<ChessPiece>,
) -> String {
    if let ChessMoveType::Castle { .. } = chess_move_type {
        return encode_move_as_long_algebraic_notation(chess_move_type);
    }

    let (
        moving_piece_color,
        moving_piece_type,
        moving_piece_original_location,
        moving_piece_new_position,
        is_move_en_passant,
        promotion,
    ) = get_move_data(chess_move_type);

    let mut game = build_game(chess_move_type, starting_position, moving_piece_color);

    let legal_moves = get_legal_moves(&mut game);

    game.make_move(chess_move_type.clone());

    let conflicts = find_conflicts(
        moving_piece_type,
        moving_piece_original_location,
        moving_piece_new_position,
        &legal_moves,
    );

    let piece_name = match moving_piece_type {
        PieceType::Pawn => "",
        PieceType::Rook => "R",
        PieceType::Knight => "N",
        PieceType::Bishop => "B",
        PieceType::Queen => "Q",
        PieceType::King => "K",
    };

    let conflicts_with_same_file = conflicts
        .iter()
        .filter(|&c| match c {
            ChessMoveType::Move {
                original_position, ..
            } => original_position.0 == moving_piece_original_location.0,
            _ => false,
        })
        .count();

    let conflicts_with_same_rank = conflicts
        .iter()
        .filter(|&c| match c {
            ChessMoveType::Move {
                original_position, ..
            } => original_position.1 == moving_piece_original_location.1,
            _ => false,
        })
        .count();

    let conflict_rank_str = if conflicts_with_same_file != 0 {
        get_rank_name(moving_piece_original_location.1)
    } else {
        String::new()
    };
    let conflict_file_str = if conflicts_with_same_rank != 0 {
        get_file_name(moving_piece_original_location.0)
    } else {
        String::new()
    };

    let conflict_string = format!("{}{}", conflict_file_str, conflict_rank_str);

    let take_string = match chess_move_type {
        ChessMoveType::Move { taken_piece, .. } => {
            if taken_piece.is_some() {
                String::from("x")
            } else {
                String::new()
            }
        }
        ChessMoveType::EnPassant {
            original_position, ..
        } => format!("{}x", get_file_name(original_position.0)),
        _ => String::new(),
    };

    let new_position_name =
        get_square_name_from_row_and_col(moving_piece_new_position.0, moving_piece_new_position.1);

    let game_state_string = match game.get_game_state() {
        GameState::Check { .. } => "+",
        GameState::Checkmate { .. } => "#",
        _ => "",
    };

    let en_passant_string = if is_move_en_passant { " e.p." } else { "" };
    let promotion_string = if let Some(promotion_piece) = promotion {
        let promotion_piece_char = match promotion_piece.get_piece_type() {
            PieceType::Pawn => "",
            PieceType::Rook => "R",
            PieceType::Knight => "N",
            PieceType::Bishop => "B",
            PieceType::Queen => "Q",
            PieceType::King => "K",
        };
        format!("={}", promotion_piece_char)
    } else {
        String::new()
    };

    format!(
        "{}{}{}{}{}{}{}",
        piece_name,
        conflict_string,
        take_string,
        new_position_name,
        promotion_string,
        game_state_string,
        en_passant_string
    )
}

fn get_move_data(
    chess_move_type: &ChessMoveType,
) -> (
    Color,
    PieceType,
    &(usize, usize),
    &(usize, usize),
    bool,
    Option<&ChessPiece>,
) {
    match chess_move_type {
        ChessMoveType::Move {
            piece,
            original_position,
            new_position,
            taken_piece,
            promotion,
        } => (
            piece.get_color(),
            piece.get_piece_type(),
            original_position,
            new_position,
            false,
            promotion.as_ref(),
        ),
        ChessMoveType::EnPassant {
            piece,
            original_position,
            new_position,
            promotion,
            ..
        } => (
            piece.get_color(),
            piece.get_piece_type(),
            original_position,
            new_position,
            true,
            promotion.as_ref(),
        ),
        _ => panic!("Unexpected move type"),
    }
}

fn get_legal_moves(game: &mut ChessGame) -> Vec<ChessMoveType> {
    match game.get_game_state() {
        GameState::InProgress { legal_moves, .. } => legal_moves,
        GameState::Check { legal_moves, .. } => legal_moves,
        _ => panic!("Unexpected state"),
    }
}

fn build_game(
    chess_move_type: &ChessMoveType,
    position: &Board<ChessPiece>,
    turn: Color,
) -> ChessGame {
    ChessGame::build(
        position.clone(),
        turn,
        1,
        0,
        false,
        false,
        false,
        false,
        vec![chess_move_type.clone()],
    )
}

fn find_conflicts<'a>(
    moving_piece_type: PieceType,
    moving_piece_original_location: &(usize, usize),
    moving_piece_new_position: &(usize, usize),
    previous_legal_moves: &'a Vec<ChessMoveType>,
) -> Vec<&'a ChessMoveType> {
    previous_legal_moves
        .iter()
        .filter(|&plm| match plm {
            ChessMoveType::Move {
                piece,
                original_position,
                new_position,
                ..
            } => {
                piece.get_piece_type() == moving_piece_type
                    && original_position != moving_piece_original_location
                    && new_position == moving_piece_new_position
            }
            _ => false,
        })
        .collect::<Vec<&ChessMoveType>>()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::codec::forsyth_edwards_notation::build_game_from_string;
    use crate::piece::PieceType::Rook;

    #[test]
    fn test_conflicts_found() {
        let starting_game_fen_string = String::from("7k/3R4/8/1R1p2R1/8/8/3R4/4K3 w - - 0 1");
        let mut game = build_game_from_string(&starting_game_fen_string).unwrap();

        let moves = match game.get_game_state() {
            GameState::InProgress { legal_moves, .. } => legal_moves,
            _ => panic!("Unexpected state"),
        };

        let m = moves
            .iter()
            .find(|&m| match m {
                ChessMoveType::Move {
                    original_position,
                    new_position,
                    ..
                } => {
                    original_position.0 == 1
                        && original_position.1 == 4
                        && new_position.0 == 3
                        && new_position.1 == 4
                }
                ChessMoveType::EnPassant { .. } => false,
                ChessMoveType::Castle { .. } => false,
            })
            .unwrap();

        game.make_move(m.clone());

        let conflicts = find_conflicts(Rook, &(1, 4), &(3, 4), &moves);
        assert_eq!(3, conflicts.len());
    }

    #[test]
    fn test_encode_ambiguous_rook_capture_move() {
        let starting_game_fen_string = String::from("7k/3R4/8/1R1p2R1/8/8/3R4/4K3 w - - 0 1");
        let mut game = build_game_from_string(&starting_game_fen_string).unwrap();

        let moves = match game.get_game_state() {
            GameState::InProgress { legal_moves, .. } => legal_moves,
            _ => panic!("Unexpected state"),
        };

        let m = moves
            .iter()
            .find(|&m| match m {
                ChessMoveType::Move {
                    original_position,
                    new_position,
                    ..
                } => {
                    original_position.0 == 1
                        && original_position.1 == 4
                        && new_position.0 == 3
                        && new_position.1 == 4
                }
                ChessMoveType::EnPassant { .. } => false,
                ChessMoveType::Castle { .. } => false,
            })
            .unwrap();

        let algebraic_notation = encode_move_as_algebraic_notation(m, &game.get_board());
        assert_eq!("Rbxd5", algebraic_notation);
    }

    #[test]
    fn test_encode_ambiguous_queen_move() {
        let starting_game_fen_string = String::from("1k1r3r/8/8/R7/4Q2Q/8/8/R1K4Q w - - 0 1");
        let mut game = build_game_from_string(&starting_game_fen_string).unwrap();

        let moves = match game.get_game_state() {
            GameState::InProgress { legal_moves, .. } => legal_moves,
            _ => panic!("Unexpected state"),
        };

        let m = moves
            .iter()
            .find(|&m| match m {
                ChessMoveType::Move {
                    original_position,
                    new_position,
                    ..
                } => {
                    original_position.0 == 7
                        && original_position.1 == 3
                        && new_position.0 == 4
                        && new_position.1 == 0
                }
                ChessMoveType::EnPassant { .. } => false,
                ChessMoveType::Castle { .. } => false,
            })
            .unwrap();

        let algebraic_notation = encode_move_as_algebraic_notation(m, &game.get_board());
        assert_eq!("Qh4e1", algebraic_notation);
    }

    #[test]
    fn test_encode_ambiguous_rook_move() {
        let starting_game_fen_string = String::from("1k1r3r/8/8/R7/4Q2Q/8/8/R1K4Q w - - 0 1");
        let mut game = build_game_from_string(&starting_game_fen_string).unwrap();

        let moves = match game.get_game_state() {
            GameState::InProgress { legal_moves, .. } => legal_moves,
            _ => panic!("Unexpected state"),
        };

        let m = moves
            .iter()
            .find(|&m| match m {
                ChessMoveType::Move {
                    original_position,
                    new_position,
                    ..
                } => {
                    original_position.0 == 0
                        && original_position.1 == 0
                        && new_position.0 == 0
                        && new_position.1 == 2
                }
                ChessMoveType::EnPassant { .. } => false,
                ChessMoveType::Castle { .. } => false,
            })
            .unwrap();

        let algebraic_notation = encode_move_as_algebraic_notation(m, &game.get_board());
        assert_eq!("R1a3", algebraic_notation);
    }

    #[test]
    fn test_encode_ambiguous_pawn_en_passant_move() {
        let starting_game_fen_string = String::from("1k6/5p2/8/4P1P1/8/8/8/1K6 b - - 0 1");
        let mut game = build_game_from_string(&starting_game_fen_string).unwrap();

        let moves = match game.get_game_state() {
            GameState::InProgress { legal_moves, .. } => legal_moves,
            _ => panic!("Unexpected state"),
        };

        let m = moves
            .iter()
            .find(|&m| match m {
                ChessMoveType::Move {
                    original_position,
                    new_position,
                    ..
                } => {
                    original_position.0 == 5
                        && original_position.1 == 6
                        && new_position.0 == 5
                        && new_position.1 == 4
                }
                ChessMoveType::EnPassant { .. } => false,
                ChessMoveType::Castle { .. } => false,
            })
            .unwrap();
        game.make_move(m.clone());

        let moves = match game.get_game_state() {
            GameState::InProgress { legal_moves, .. } => legal_moves,
            _ => panic!("Unexpected state"),
        };

        let m = moves
            .iter()
            .find(|&m| match m {
                ChessMoveType::EnPassant {
                    original_position,
                    new_position,
                    ..
                } => {
                    original_position.0 == 6
                        && original_position.1 == 4
                        && new_position.0 == 5
                        && new_position.1 == 5
                }
                _ => false,
            })
            .unwrap();

        let algebraic_notation = encode_move_as_algebraic_notation(m, &game.get_board());

        game.make_move(m.clone());
        println!("{}", game.get_board());
        assert_eq!("gxf6 e.p.", algebraic_notation);
    }

    #[test]
    fn test_encode_checkmate() {
        let starting_game_fen_string = String::from("k7/pp6/8/2q5/8/8/P7/K7 b - - 0 1");
        let mut game = build_game_from_string(&starting_game_fen_string).unwrap();

        let moves = match game.get_game_state() {
            GameState::InProgress { legal_moves, .. } => legal_moves,
            _ => panic!("Unexpected state"),
        };

        let m = moves
            .iter()
            .find(|&m| match m {
                ChessMoveType::Move {
                    original_position,
                    new_position,
                    ..
                } => {
                    original_position.0 == 2
                        && original_position.1 == 4
                        && new_position.0 == 2
                        && new_position.1 == 0
                }
                ChessMoveType::EnPassant { .. } => false,
                ChessMoveType::Castle { .. } => false,
            })
            .unwrap();

        let algebraic_notation = encode_move_as_algebraic_notation(m, &game.get_board());
        assert_eq!("Qc1#", algebraic_notation);
    }

    #[test]
    fn test_encode_check() {
        let starting_game_fen_string = String::from("k7/pp6/8/2r5/8/8/P7/K7 b - - 0 1");
        let mut game = build_game_from_string(&starting_game_fen_string).unwrap();

        let moves = match game.get_game_state() {
            GameState::InProgress { legal_moves, .. } => legal_moves,
            _ => panic!("Unexpected state"),
        };

        let m = moves
            .iter()
            .find(|&m| match m {
                ChessMoveType::Move {
                    original_position,
                    new_position,
                    ..
                } => {
                    original_position.0 == 2
                        && original_position.1 == 4
                        && new_position.0 == 2
                        && new_position.1 == 0
                }
                ChessMoveType::EnPassant { .. } => false,
                ChessMoveType::Castle { .. } => false,
            })
            .unwrap();

        let algebraic_notation = encode_move_as_algebraic_notation(m, &game.get_board());
        assert_eq!("Rc1+", algebraic_notation);
    }
}
