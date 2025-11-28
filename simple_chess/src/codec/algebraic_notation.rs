use crate::chess_game_state_analyzer::GameState;
use crate::piece::{ChessPiece, PieceType};
use crate::{ChessGame, ChessMoveType, Color};
use game_board::Board;

fn encode_move_as_algebraic_notation(
    chess_move_type: &ChessMoveType,
    starting_position: &Board<ChessPiece>,
) {
    let (
        moving_piece_color,
        moving_piece_type,
        moving_piece_original_location,
        moving_piece_new_position,
        is_move_en_passant,
        is_move_a_take,
        promotion,
    ) = get_move_data(chess_move_type);

    let mut game = build_game(chess_move_type, starting_position, moving_piece_color);

    let legal_moves = get_legal_moves(&mut game);

    let new_game_state = game.make_move(chess_move_type.clone());

    let conflicts = find_conflicts(
        moving_piece_type,
        moving_piece_original_location,
        moving_piece_new_position,
        &legal_moves,
    );
}

fn get_move_data(
    chess_move_type: &ChessMoveType,
) -> (
    Color,
    PieceType,
    &(usize, usize),
    &(usize, usize),
    bool,
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
            taken_piece.is_some(),
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
    resulting_position: &Board<ChessPiece>,
    moving_piece_color: Color,
) -> ChessGame {
    ChessGame::build(
        resulting_position.clone(),
        moving_piece_color.opposite(),
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
        println!("{:?}", conflicts);
    }
}
