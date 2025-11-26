use crate::chess_game_state_analyzer::GameState;
use crate::codec::long_algebraic_notation::encode_move_as_long_algebraic_notation;
use crate::piece::ChessPiece;
use crate::{ChessGame, ChessMoveType};
use game_board::Board;

pub fn encode_move_as_algebraic_notation(
    chess_move_type: &ChessMoveType,
    resulting_position: &Board<ChessPiece>,
) -> String {
    match chess_move_type {
        ChessMoveType::Move {
            original_position,
            new_position,
            piece,
            taken_piece,
            promotion,
        } => encode_move_as_long_algebraic_notation(chess_move_type),
        ChessMoveType::Castle { .. } => encode_move_as_long_algebraic_notation(chess_move_type),
        ChessMoveType::EnPassant {
            original_position,
            new_position,
            piece,
            taken_piece,
            taken_piece_position,
            promotion,
        } => encode_move_as_long_algebraic_notation(chess_move_type),
    }
}

fn idk(chess_move_type: &ChessMoveType, resulting_position: &Board<ChessPiece>) {
    let (
        moving_piece_color,
        moving_piece_type,
        moving_piece_original_location,
        moving_piece_new_position,
    ) = match chess_move_type {
        ChessMoveType::Move {
            piece,
            original_position,
            new_position,
            ..
        } => (
            piece.get_color(),
            piece.get_piece_type(),
            original_position,
            new_position,
        ),
        ChessMoveType::EnPassant {
            piece,
            original_position,
            new_position,
            ..
        } => (
            piece.get_color(),
            piece.get_piece_type(),
            original_position,
            new_position,
        ),
        _ => panic!("Unexpected move type"),
    };

    let mut game = ChessGame::build(
        resulting_position.clone(),
        moving_piece_color.opposite(),
        1,
        0,
        false,
        false,
        false,
        false,
        vec![chess_move_type.clone()],
    );

    game.undo_last_move();

    let previous_state = &game.get_game_state();
    let previous_legal_moves = match previous_state {
        GameState::InProgress { legal_moves, .. } => legal_moves,
        GameState::Check { legal_moves, .. } => legal_moves,
        _ => panic!("Unexpected state"),
    };

    let conflicts = previous_legal_moves
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
        .collect::<Vec<&ChessMoveType>>();

    println!("{:?}", conflicts);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::codec::forsyth_edwards_notation::{
        build_game_from_string, ForsythEdwardsNotationError,
    };

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

        idk(m, &game.get_board());
    }
}
