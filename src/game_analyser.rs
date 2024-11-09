use crate::{Color, Game, PieceType};
use crate::chess_board::GameBoard;
use crate::chess_move::ChessMove;

fn get_all_moves(game: &mut Game) -> Vec<ChessMove> {

    let width = game.board.get_width();
    let height = game.board.get_width();

    let current_turn = game.current_turn;
    let mut legal_moves: Vec<ChessMove> = Vec::new();
    let board = &mut game.board;


    for col in 0..width {
        for row in 0..height {
            if let Some(piece) = board.check_space(col, row) {
                if piece.color == current_turn {
                    let moves = piece.get_legal_moves(col, row, board);

                    for m in moves {
                        let mut cloned_board = board.clone();
                        cloned_board.remove_piece(m.original_position.0, m.original_position.1);
                        cloned_board.place_piece(*m.piece, m.new_position.0, m.new_position.1);

                        if !is_color_in_check(&cloned_board, current_turn) {
                            legal_moves.push(m);
                        }
                    }
                }
            }
        }
    }
    legal_moves
}

fn is_color_in_check(board: &GameBoard, color: Color) -> bool {
    let opposite_color = color.opposite_color();

    for col in 0 .. board.get_width() {
        for row in 0 .. board.get_height() {
            if let Some(piece) = board.check_space(col, row) {
                if piece.color == opposite_color{
                    let moves = piece.get_legal_moves(col, row, board);
                    for mov in moves {
                        if let Some(takes_piece) = mov.takes {
                            if takes_piece.get_piece_type() == &PieceType::King &&
                                takes_piece.get_color() == &color {
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
    use crate::chess_board::GameBoard;
    use crate::Color::{Black, White};
    use super::*;

    #[test]
    fn idk() {
        let board = GameBoard::from_string(2,2, concat!(" ♛\n", "♔ ", )).unwrap();
        let is_in_check = is_color_in_check(&board, Color::White);

        assert_eq!(true, is_in_check)
    }

    #[test]
    fn more () {
        let mut game = Game::new_chess_game();

        let moves = get_all_moves(&mut game);

        for v in moves {
            println!("{v}");
        }
    }

    #[test]
    fn test_legal_moves() {
        let board = concat!(
            "  ♔  \n",
            "  ♗  \n",
            "     \n",
            "     \n",
            "  ♜  ",
        );
        let game_board = GameBoard::from_string(5, 5, board).unwrap();

        let mut game = Game::new_game(game_board, White);

        let moves = get_all_moves(&mut game);

        for m in moves {
            println!("{m}");
        }
    }

    #[test]
    fn test_legal_moves2() {
        let board = concat!(
        "  ♔  \n",
        "     \n",
        "     \n",
        " ♗   \n",
        "  ♜  ",
        );
        let game_board = GameBoard::from_string(5, 5, board).unwrap();

        let mut game = Game::new_game(game_board, White);

        let moves = get_all_moves(&mut game);

        for m in moves {
            println!("{m}");
        }
    }
}
