extern crate game_board;
mod model;

#[cfg(test)]
mod tests {
    use crate::model::CheckersPiece;

    #[test]
    fn simple_board_with_piece_test() {
        let mut board = game_board::Board::build(1, 1).unwrap();
        assert_eq!(1, board.get_width());
        assert_eq!(1, board.get_height());

        assert!(board.get_piece_at_space(0, 0).is_none());

        let piece = CheckersPiece::new();
        board.place_piece(piece, 0, 0);

        let piece_from_board = board.get_piece_at_space(0, 0);
        assert!(piece_from_board.is_some());

        let removed_piece = board.remove_piece(0, 0);
        assert!(removed_piece.is_some());
        assert!(board.get_piece_at_space(0, 0).is_none());
    }
}
