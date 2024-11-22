extern crate game_board;
mod model;

#[cfg(test)]
mod tests {
    use crate::model::CheckersPiece;
    use std::ops::Deref;

    #[test]
    fn simple_board_with_piece_test() {
        let mut board = game_board::Board::build(1, 1).unwrap();
        assert_eq!(1, board.get_width());
        assert_eq!(1, board.get_height());

        assert!(board.get_piece_at_space(0, 0).is_none());

        let piece = Box::new(CheckersPiece::new());
        board.place_piece(piece, 0, 0);

        let piece_from_board = board.get_piece_at_space(0, 0).unwrap();
        if piece_from_board
            .deref()
            .as_any()
            .downcast_ref::<CheckersPiece>()
            .is_none()
        {
            panic!("Expected Checkers Piece")
        }

        let removed_piece = board.remove_piece(0, 0);
        assert!(removed_piece.is_some());
        if removed_piece
            .unwrap()
            .as_any()
            .downcast_ref::<CheckersPiece>()
            .is_none()
        {
            panic!("Expected Checkers Piece")
        }

        assert!(board.get_piece_at_space(0, 0).is_none());
    }
}
