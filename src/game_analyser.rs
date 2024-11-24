use crate::chess_board::GameBoard;
use crate::chess_board_square::SquareId;
use crate::chess_move::ChessMoveType;
use crate::game_state::GameState;
use crate::game_state::GameState::{
    Check, Checkmate, DrawByRepetition, FiftyMoveRule, InProgress, InsufficientMaterial, Stalemate,
};
use crate::Color::{Black, White};
use crate::PieceType::{Bishop, King, Knight, Rook};
use crate::{ChessPiece, Color, Game};

pub fn get_game_state(game: &Game) -> (GameState, Vec<ChessMoveType>) {
    let is_in_check =
        is_color_in_check(game.get_board(), game.current_turn, game.get_moves().last());
    let possible_next_moves = get_all_moves(game);

    if possible_next_moves.is_empty() {
        return if is_in_check {
            (Checkmate, possible_next_moves)
        } else {
            (Stalemate, possible_next_moves)
        };
    }

    if is_in_check {
        return (Check, possible_next_moves);
    }

    let mut active_white_pieces = Vec::new();
    let mut active_black_pieces = Vec::new();

    for col in 0..game.get_board().get_width() {
        for row in 0..game.get_board().get_height() {
            let piece = game.get_board().check_space(col, row);
            if let Some(piece) = piece {
                if piece.color == White {
                    active_white_pieces.push(piece);
                } else {
                    active_black_pieces.push(piece);
                }
            }
        }
    }

    if is_insufficient_material(active_white_pieces)
        && is_insufficient_material(active_black_pieces)
    {
        return (InsufficientMaterial, possible_next_moves);
    }

    if game.can_trigger_fifty_move_rule() {
        return (FiftyMoveRule, possible_next_moves);
    }

    if game.can_trigger_draw_by_repetition() {
        return (DrawByRepetition, possible_next_moves);
    }

    (InProgress, possible_next_moves)
}

fn is_insufficient_material(pieces: Vec<&ChessPiece>) -> bool {
    let king_count = pieces.iter().filter(|p| p.piece_type == King).count();
    let bishop_count = pieces.iter().filter(|p| p.piece_type == Bishop).count();
    let knight_count = pieces.iter().filter(|p| p.piece_type == Knight).count();
    let other_count = pieces.len() - knight_count - bishop_count - king_count;

    if other_count > 0 {
        return false;
    }

    if king_count == 0 {
        panic!("No king?");
    }

    if bishop_count == 0 && knight_count == 0 {
        return true;
    }

    if bishop_count == 1 && knight_count == 0 {
        return true;
    }

    if bishop_count == 0 && knight_count == 1 {
        return true;
    }

    false
}

fn get_all_moves(game: &Game) -> Vec<ChessMoveType> {
    let width = game.board.get_width();
    let height = game.board.get_height();

    let current_turn = game.current_turn;
    let mut legal_moves: Vec<ChessMoveType> = Vec::new();
    let board = game.get_board();

    for col in 0..width {
        for row in 0..height {
            if let Some(piece) = board.check_space(col, row) {
                if piece.color == current_turn {
                    let moves = piece.get_legal_moves(col, row, board, game.get_moves().last());

                    for m in moves {
                        let mut cloned_board = board.clone();
                        m.make_move(&mut cloned_board);

                        if !is_color_in_check(&cloned_board, current_turn, game.get_moves().last())
                        {
                            legal_moves.push(m);
                        }
                    }
                }
            }
        }
    }

    match current_turn {
        White => {
            if game.white_can_castle_long {
                if let Some(m) = can_castle_long(White, board) {
                    legal_moves.push(m);
                }
            }
            if game.white_can_castle_short {
                if let Some(m) = can_castle_short(White, board) {
                    legal_moves.push(m);
                }
            }
        }
        Black => {
            if game.black_can_castle_long {
                if let Some(m) = can_castle_long(Black, board) {
                    legal_moves.push(m);
                }
            }
            if game.black_can_castle_short {
                if let Some(m) = can_castle_short(Black, board) {
                    legal_moves.push(m);
                }
            }
        }
    }

    legal_moves
}

fn can_castle_long(color: Color, board: &GameBoard) -> Option<ChessMoveType> {
    let row = match color {
        White => 0,
        Black => board.get_height() - 1,
    };

    if is_color_in_check(board, color, None) {
        return None;
    }

    if let Some(piece) = board.check_space(0, row) {
        if piece.piece_type != Rook {
            return None;
        }
    } else {
        return None;
    }

    let mut king_col = None;
    for col in 1..board.get_width() {
        if let Some(piece) = board.check_space(col, row) {
            if piece.piece_type != King {
                return None;
            } else {
                king_col = Some(col);
                break;
            }
        }
    }

    if king_col? < 2 {
        return None;
    }

    let mut board_clone = board.clone();
    let king = board_clone.remove_piece(king_col?, row).unwrap();
    board_clone.place_piece(king, king_col? - 1, row);
    if is_color_in_check(&board_clone, color, None) {
        return None;
    }

    let mut board_clone = board.clone();
    let king = board_clone.remove_piece(king_col?, row).unwrap();
    board_clone.place_piece(king, king_col? - 2, row);
    if is_color_in_check(&board_clone, color, None) {
        return None;
    }

    Some(ChessMoveType::Castle {
        king: *board.check_space(king_col?, row).unwrap(),
        original_king_position: SquareId::build(king_col?, row),
        new_king_position: SquareId::build(king_col? - 2, row),
        rook: *board.check_space(0, row).unwrap(),
        original_rook_position: SquareId::build(0, row),
        new_rook_position: SquareId::build(king_col? - 1, row),
    })
}

fn can_castle_short(color: Color, board: &GameBoard) -> Option<ChessMoveType> {
    let row = match color {
        White => 0,
        Black => board.get_height() - 1,
    };

    if is_color_in_check(board, color, None) {
        return None;
    }

    if let Some(piece) = board.check_space(board.get_width() - 1, row) {
        if piece.piece_type != Rook {
            return None;
        }
    } else {
        return None;
    }

    let range = 0..board.get_width() - 1;
    let range = range.rev();

    let mut king_col = None;
    for col in range {
        if let Some(piece) = board.check_space(col, row) {
            if piece.piece_type != King {
                return None;
            } else {
                king_col = Some(col);
                break;
            }
        };
    }

    if board.get_width() < 3 || king_col? > (board.get_width() - 3) {
        return None;
    }

    let mut board_clone = board.clone();
    let king = board_clone.remove_piece(king_col?, row).unwrap();
    board_clone.place_piece(king, king_col? + 1, row);
    if is_color_in_check(&board_clone, color, None) {
        return None;
    }
    let mut board_clone = board.clone();
    let king = board_clone.remove_piece(king_col?, row).unwrap();
    board_clone.place_piece(king, king_col? + 2, row);
    if is_color_in_check(&board_clone, color, None) {
        return None;
    }

    Some(ChessMoveType::Castle {
        king: *board.check_space(king_col?, row).unwrap(),
        original_king_position: SquareId::build(king_col?, row),
        new_king_position: SquareId::build(king_col? + 2, row),
        rook: *board.check_space(board.get_width() - 1, row).unwrap(),
        original_rook_position: SquareId::build(board.get_width() - 1, row),
        new_rook_position: SquareId::build(king_col? + 1, row),
    })
}

fn is_color_in_check(board: &GameBoard, color: Color, last_move: Option<&ChessMoveType>) -> bool {
    let opposite_color = color.opposite_color();

    for col in 0..board.get_width() {
        for row in 0..board.get_height() {
            if let Some(piece) = board.check_space(col, row) {
                if piece.color == opposite_color {
                    let moves = piece.get_legal_moves(col, row, board, last_move);
                    for mov in moves {
                        if let ChessMoveType::Move {
                            taken_piece: Some(taken_piece),
                            ..
                        } = mov
                        {
                            if taken_piece.get_piece_type() == &King
                                && taken_piece.get_color() == &color
                            {
                                return true;
                            }
                        }
                    }
                }
            }
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chess_board::GameBoard;
    use crate::Color::{Black, White};

    #[test]
    fn idk() {
        let board = GameBoard::from_string(2, 2, concat!(" ♛\n", "♔ ",)).unwrap();
        let is_in_check = is_color_in_check(&board, Color::White, None);

        assert!(is_in_check)
    }

    #[test]
    fn more() {
        let game = Game::new_chess_game();

        let moves = get_all_moves(&game);

        for v in moves {
            println!("{v}");
        }
    }

    #[test]
    fn test_legal_moves() {
        let board = concat!("  ♔  \n", "  ♗  \n", "     \n", "     \n", "  ♜  ",);
        let game_board = GameBoard::from_string(5, 5, board).unwrap();

        let game = Game::new_game(game_board, White);

        let moves = get_all_moves(&game);

        for m in moves {
            println!("{m}");
        }
    }

    #[test]
    fn test_legal_moves2() {
        let board = concat!("  ♔  \n", "     \n", "♗    \n", "     \n", " ♜♜♜ ",);
        let game_board = GameBoard::from_string(5, 5, board).unwrap();

        let game = Game::new_game(game_board, White);

        let moves = get_all_moves(&game);

        for m in moves {
            println!("{m}");
        }
    }

    #[test]
    fn test_legal_moves3() {
        let chess_board_as_string = concat!(
            "♜♞♝ ♚♝♞♜\n",
            "♟♟♟♟♟♟♟♟\n",
            "        \n",
            "        \n",
            "      ♙♛\n",
            "     ♙  \n",
            "♙♙♙♙♙  ♙\n",
            "♖♘♗♕♔♗♘♖\n"
        );
        let game_board = GameBoard::from_string(8, 8, chess_board_as_string).unwrap();

        let game = Game::new_game(game_board, White);

        let moves = get_all_moves(&game);

        assert_eq!(0, moves.len());
    }

    #[test]
    fn forced_move() {
        let chess_board_as_string = concat!(
            "  ♚♜    \n",
            "♟♟♟ ♘   \n",
            "  ♝   ♟ \n",
            "    ♙♟  \n",
            " ♙    ♙♜\n",
            "♙   ♔  ♙\n",
            "  ♙     \n",
            "   ♖   ♖\n"
        );
        let game_board = GameBoard::from_string(8, 8, chess_board_as_string).unwrap();

        println!("{game_board}");

        let game = Game::new_game(game_board, Black);

        let moves = get_all_moves(&game);

        assert_eq!(1, moves.len());
        let only_move = moves.first().unwrap();

        if let ChessMoveType::Move { new_position, .. } = only_move {
            assert_eq!(1, new_position.get_column());
            assert_eq!(7, new_position.get_row());
        } else {
            panic!("Should be a move")
        }

        println!("{only_move}");
    }

    #[test]
    fn test_en_passant() {
        let chess_board_as_string = concat!("♚ \n", "♟ \n", "  \n", " ♙\n", " ♔");
        let game_board = GameBoard::from_string(2, 5, chess_board_as_string).unwrap();

        let mut game = Game::new_game(game_board, White);

        println!("This is the board\n{}", game.get_board());

        let (_, next_moves) = get_game_state(&game);

        for m in &next_moves {
            println!("{m}");
        }

        let move_pawn_to_b4 = next_moves
            .iter()
            .find(|p| -> bool {
                if let ChessMoveType::Move { new_position, .. } = p {
                    if new_position.get_column() == 1 && new_position.get_row() == 3 {
                        return true;
                    }
                };
                false
            })
            .unwrap();
        game.make_move(move_pawn_to_b4);

        println!("{}", game.get_board());
        let (state, moves) = get_game_state(&game);

        println!("{:?}", state);

        for m in &moves {
            println!("{m}");
        }

        assert_eq!(3, moves.len());
    }

    #[test]
    fn test_en_passant2() {
        let chess_board_as_string = concat!(" ♚\n", " ♟\n", "  \n", "♙ \n", " ♔");
        let game_board = GameBoard::from_string(2, 5, chess_board_as_string).unwrap();

        let mut game = Game::new_game(game_board, White);

        println!("This is the board\n{}", game.get_board());

        let (_, next_moves) = get_game_state(&game);

        for m in &next_moves {
            println!("{m}");
        }

        let move_pawn_to_b4 = next_moves
            .iter()
            .find(|p| -> bool {
                if let ChessMoveType::Move { new_position, .. } = p {
                    if new_position.get_column() == 0 && new_position.get_row() == 3 {
                        return true;
                    }
                }
                false
            })
            .unwrap();

        game.make_move(move_pawn_to_b4);

        println!("{}", game.get_board());
        let (state, moves) = get_game_state(&game);

        println!("{:?}", state);

        for m in &moves {
            println!("{m}");
        }

        assert_eq!(3, moves.len());
    }

    #[test]
    fn can_black_castle_long() {
        let chess_board_as_string = concat!(
            "♜   ♚ ♞♜\n",
            "♟♟♟♟♟♟♟♟\n",
            "        \n",
            "        \n",
            "      ♙♛\n",
            "     ♙  \n",
            "♙♙♙♙♙  ♙\n",
            "♖♘♗♕♔♗♘♖\n"
        );
        let game_board = GameBoard::from_string(8, 8, chess_board_as_string).unwrap();

        let can_castle_long = can_castle_long(Black, &game_board);

        assert!(can_castle_long.is_some())
    }

    #[test]
    fn can_white_castle_long() {
        let chess_board_as_string = concat!(
            "♜ ♞ ♚ ♞♜\n",
            "♟♟♟♟♟♟♟♟\n",
            "        \n",
            "        \n",
            "     ♛  \n",
            "        \n",
            "♙♙♙♙   ♙\n",
            "♖   ♔♗♘♖"
        );
        let game_board = GameBoard::from_string(8, 8, chess_board_as_string).unwrap();

        let can_castle_long = can_castle_long(White, &game_board);

        assert!(can_castle_long.is_some())
    }

    #[test]
    fn cant_black_castle_short() {
        let chess_board_as_string = concat!(
            "♜  ♞♚  ♜\n",
            "♟♟♟♟♟♟♟♟\n",
            "        \n",
            "        \n",
            "      ♙♛\n",
            "     ♙  \n",
            "♙♙♙♙♙  ♙\n",
            "♖♘♗♕♔♗♘♖\n"
        );
        let game_board = GameBoard::from_string(8, 8, chess_board_as_string).unwrap();

        let can_castle_short = can_castle_short(Black, &game_board);

        assert!(can_castle_short.is_some())
    }

    #[test]
    fn cant_white_castle_short() {
        let chess_board_as_string = concat!(
            "♜  ♞♚  ♜\n",
            "♟♟♟♟♟♟♟♟\n",
            "        \n",
            "        \n",
            "      ♙♛\n",
            "        \n",
            "♙♙♙♙♙♙ ♙\n",
            "♖♘♗♕♔  ♖\n"
        );
        let game_board = GameBoard::from_string(8, 8, chess_board_as_string).unwrap();

        let can_castle_short = can_castle_short(White, &game_board);

        assert!(can_castle_short.is_some())
    }

    #[test]
    fn solve_this_bug() {
        let board_as_string = concat!("♔ \n", "  \n", "♟♚\n", "  ");

        let game_board = GameBoard::from_string(2, 4, board_as_string).unwrap();

        is_color_in_check(&game_board, Color::White, None);
    }
}
