use crate::board::Board;
use crate::board_square::BoardSquare;
use crate::pieces::better_piece::{ChessPiece, Piece, PieceColor};
use crate::pieces::better_piece::ChessPiece::*;

struct Game<'a> {
    chess_board: Board<'a>,
    white_pieces: Vec<Piece>,
    black_pieces: Vec<Piece>,
    current_players_move: PieceColor,
}

impl<'a> Game<'_> {
    fn new() -> Game<'a> {
        let mut board = Board::new(8, 8);
        let mut white_pieces = Self::create_pieces(PieceColor::White);
        let mut black_pieces = Self::create_pieces(PieceColor::Black);

        place_pieces_in_starting_positions(&mut board, &mut white_pieces, &mut black_pieces);

        Game {
            chess_board: board,
            white_pieces,
            black_pieces,
            current_players_move: PieceColor::White,
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

fn place_pieces_in_starting_positions(board: &mut Board, white_pieces: &mut Vec<Piece>, black_pieces: &mut Vec<Piece>) {

}

fn set_piece_on_space<'a>(piece: &'a mut Piece, space: &mut BoardSquare<'a>) {
    piece.board_square_id = Some(space.name.clone());
    space.set_piece(piece);
}