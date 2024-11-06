use crate::{Game, PieceType};

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

