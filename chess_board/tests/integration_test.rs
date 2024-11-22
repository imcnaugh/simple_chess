extern crate chess_board;
mod model;

#[cfg(test)]
mod tests {
    use std::ops::Deref;
    use crate::model::CheckersPiece;

    #[test]
    fn simple_board_with_piece_test() {
        let mut board = chess_board::Board::build(1, 1).unwrap();
        assert_eq!(1, board.get_width());
        assert_eq!(1, board.get_height());

        assert!(board.check_space(0, 0).is_none());

        let piece = Box::new(CheckersPiece::new());
        board.place_piece(piece, 0, 0);

        let piece_from_board = board.check_space(0, 0).unwrap();
        if piece_from_board.deref().as_any().downcast_ref::<CheckersPiece>().is_none() {
            panic!("Expected Checkers Piece")
        }
        
        let removed_piece = board.remove_piece(0, 0);
        assert!(removed_piece.is_some());
        if removed_piece.unwrap().as_any().downcast_ref::<CheckersPiece>().is_none() {
            panic!("Expected Checkers Piece")
        }
        
        assert!(board.check_space(0, 0).is_none());
    }
}
