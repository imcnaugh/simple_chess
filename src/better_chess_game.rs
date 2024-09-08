use crate::board::Board;
use crate::pieces::better_piece::{ChessPiece, Piece, PieceColor};
use crate::pieces::better_piece::ChessPiece::*;

struct Game<'a> {
    chess_board: Board<'a>,
    white_pieces: Vec<Piece>,
    black_pieces: Vec<Piece>,
}

impl<'a> Game<'_> {
    fn new() -> Game<'a> {
        let board = Board::new(8, 8);
        let white_pieces = Self::create_pieces(PieceColor::White);
        let black_pieces = Self::create_pieces(PieceColor::Black);

        Game {
            chess_board: board,
            white_pieces,
            black_pieces,
        }
    }

    fn create_pieces(color: PieceColor) -> Vec<Piece> {
        let create_piece = |t: ChessPiece| -> Piece {
            Piece {
                piece: t,
                color: color.clone(),
                board_square_id: None,
            }
        };

        vec![
            create_piece(King { _has_moved: false }),
            create_piece(Queen),
            create_piece(Rook { _has_moved: false }),
            create_piece(Rook { _has_moved: false }),
            create_piece(Bishop),
            create_piece(Bishop),
            create_piece(Knight),
            create_piece(Knight),
            create_piece(Pawn { _has_moved: false }),
            create_piece(Pawn { _has_moved: false }),
            create_piece(Pawn { _has_moved: false }),
            create_piece(Pawn { _has_moved: false }),
            create_piece(Pawn { _has_moved: false }),
            create_piece(Pawn { _has_moved: false }),
            create_piece(Pawn { _has_moved: false }),
            create_piece(Pawn { _has_moved: false }),
        ]
    }
}