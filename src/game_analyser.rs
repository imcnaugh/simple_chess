use crate::chess_board::GameBoard;
use crate::chess_move::ChessMove;
use crate::game_state::GameState;
use crate::game_state::GameState::{
    Check, Checkmate, FiftyMoveRule, InProgress, InsufficientMaterial, Stalemate,
};
use crate::Color::White;
use crate::PieceType::{Bishop, King, Knight, Pawn};
use crate::{ChessPiece, Color, Game, PieceType};

pub fn get_game_state(game: &Game) -> (GameState, Vec<ChessMove>) {
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

fn get_all_moves(game: &Game) -> Vec<ChessMove> {
    let width = game.board.get_width();
    let height = game.board.get_height();

    let current_turn = game.current_turn;
    let mut legal_moves: Vec<ChessMove> = Vec::new();
    let board = game.get_board();

    for col in 0..width {
        for row in 0..height {
            if let Some(piece) = board.check_space(col, row) {
                if piece.color == current_turn {
                    let moves = piece.get_legal_moves(col, row, board, game.get_moves().last());

                    for m in moves {
                        let mut cloned_board = board.clone();
                        if let Some(taken_position) = m.taken_piece_position {
                            cloned_board.remove_piece(taken_position.0, taken_position.1);
                        }
                        cloned_board.remove_piece(m.original_position.0, m.original_position.1);
                        cloned_board.place_piece(m.piece, m.new_position.0, m.new_position.1);

                        if !is_color_in_check(&cloned_board, current_turn, game.get_moves().last())
                        {
                            legal_moves.push(m);
                        }
                    }
                }
            }
        }
    }

    legal_moves
}

fn is_color_in_check(board: &GameBoard, color: Color, last_move: Option<&ChessMove>) -> bool {
    let opposite_color = color.opposite_color();

    for col in 0..board.get_width() {
        for row in 0..board.get_height() {
            if let Some(piece) = board.check_space(col, row) {
                if piece.color == opposite_color {
                    let moves = piece.get_legal_moves(col, row, board, last_move);
                    for mov in moves {
                        if let Some(takes_piece) = mov.takes {
                            if takes_piece.get_piece_type() == &King
                                && takes_piece.get_color() == &color
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

        assert_eq!(true, is_in_check)
    }

    #[test]
    fn more() {
        let mut game = Game::new_chess_game();

        let moves = get_all_moves(&mut game);

        for v in moves {
            println!("{v}");
        }
    }

    #[test]
    fn test_legal_moves() {
        let board = concat!("  ♔  \n", "  ♗  \n", "     \n", "     \n", "  ♜  ",);
        let game_board = GameBoard::from_string(5, 5, board).unwrap();

        let mut game = Game::new_game(game_board, White);

        let moves = get_all_moves(&mut game);

        for m in moves {
            println!("{m}");
        }
    }

    #[test]
    fn test_legal_moves2() {
        let board = concat!("  ♔  \n", "     \n", "♗    \n", "     \n", " ♜♜♜ ",);
        let game_board = GameBoard::from_string(5, 5, board).unwrap();

        let mut game = Game::new_game(game_board, White);

        let moves = get_all_moves(&mut game);

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

        let mut game = Game::new_game(game_board, White);

        let moves = get_all_moves(&mut game);

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

        let mut game = Game::new_game(game_board, Black);

        let moves = get_all_moves(&mut game);

        assert_eq!(1, moves.len());
        let only_move = moves.first().unwrap();
        assert_eq!(1, only_move.new_position.0);
        assert_eq!(7, only_move.new_position.1);

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

        let move_pawn_to_b4 = *next_moves
            .iter()
            .find(|p| p.new_position.0 == 1 && p.new_position.1 == 3)
            .unwrap();
        game.get_board_mut().remove_piece(
            move_pawn_to_b4.original_position.0,
            move_pawn_to_b4.original_position.1,
        );
        game.get_board_mut().place_piece(
            move_pawn_to_b4.piece,
            move_pawn_to_b4.new_position.0,
            move_pawn_to_b4.new_position.1,
        );

        game.change_turn(move_pawn_to_b4);

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

        let move_pawn_to_b4 = *next_moves
            .iter()
            .find(|p| p.new_position.0 == 0 && p.new_position.1 == 3)
            .unwrap();
        game.get_board_mut().remove_piece(
            move_pawn_to_b4.original_position.0,
            move_pawn_to_b4.original_position.1,
        );
        game.get_board_mut().place_piece(
            move_pawn_to_b4.piece,
            move_pawn_to_b4.new_position.0,
            move_pawn_to_b4.new_position.1,
        );

        game.change_turn(move_pawn_to_b4);

        println!("{}", game.get_board());
        let (state, moves) = get_game_state(&game);

        println!("{:?}", state);

        for m in &moves {
            println!("{m}");
        }

        assert_eq!(3, moves.len());
    }
}
