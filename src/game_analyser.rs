use crate::{Color, Game, PieceType};
use crate::chess_board::GameBoard;

pub fn get_legal_moves(game: &Game) {
    let current_turn = game.current_turn;

    let mut in_check = false;

    for col in 0..8 {
        for row in 0..8 {
            let piece = game.board.check_space(col, row);
            if let Some(piece) = piece {
                if piece.color == current_turn {
                    let moves = piece.get_legal_moves(col, row, &game.board);
                }
            }
        }
    }
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
    use super::*;

    #[test]
    fn idk() {
        let board = GameBoard::from_string(2,2, concat!(" ♛\n", "♔ ", )).unwrap();
        let is_in_check = is_color_in_check(&board, Color::White);

        assert_eq!(true, is_in_check)
    }
}
