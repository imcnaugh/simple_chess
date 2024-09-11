use crate::chess_piece::{ChessPiece, PieceType};
use crate::{ChessBoard, Color};

pub struct ChessGame {
    pub board: ChessBoard,
    pub(crate) current_turn: Color,
    turn_number: u32,
    moves: Vec<String>,
}

impl ChessGame {
    pub fn new() -> ChessGame {
        let board = ChessGame::create_board_with_starting_position();

        ChessGame {
            board,
            current_turn: Color::White,
            turn_number: 1,
            moves: vec![],
        }
    }

    fn change_turn(&mut self) {
        self.current_turn = match self.current_turn {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }

    fn create_board_with_starting_position() -> ChessBoard {
        let mut board = ChessBoard::generate_chess_board();

        for i in 0..8 {
            board.place_piece(
                ChessPiece::new(Color::White, PieceType::Pawn { has_moved: false }),
                i,
                1,
            );
            board.place_piece(
                ChessPiece::new(Color::Black, PieceType::Pawn { has_moved: false }),
                i,
                6,
            );
        }

        board.place_piece(
            ChessPiece::new(Color::White, PieceType::Rook { has_moved: false }),
            0,
            0,
        );
        board.place_piece(
            ChessPiece::new(Color::White, PieceType::Rook { has_moved: false }),
            7,
            0,
        );
        board.place_piece(
            ChessPiece::new(Color::Black, PieceType::Rook { has_moved: false }),
            0,
            7,
        );
        board.place_piece(
            ChessPiece::new(Color::Black, PieceType::Rook { has_moved: false }),
            7,
            7,
        );
        board.place_piece(ChessPiece::new(Color::White, PieceType::Knight), 1, 0);
        board.place_piece(ChessPiece::new(Color::White, PieceType::Knight), 6, 0);
        board.place_piece(ChessPiece::new(Color::Black, PieceType::Knight), 1, 7);
        board.place_piece(ChessPiece::new(Color::Black, PieceType::Knight), 6, 7);
        board.place_piece(ChessPiece::new(Color::White, PieceType::Bishop), 2, 0);
        board.place_piece(ChessPiece::new(Color::White, PieceType::Bishop), 5, 0);
        board.place_piece(ChessPiece::new(Color::Black, PieceType::Bishop), 2, 7);
        board.place_piece(ChessPiece::new(Color::Black, PieceType::Bishop), 5, 7);
        board.place_piece(ChessPiece::new(Color::White, PieceType::Queen), 3, 0);
        board.place_piece(ChessPiece::new(Color::Black, PieceType::Queen), 3, 7);
        board.place_piece(
            ChessPiece::new(Color::White, PieceType::King { has_moved: false }),
            4,
            0,
        );
        board.place_piece(
            ChessPiece::new(Color::Black, PieceType::King { has_moved: false }),
            4,
            7,
        );

        board
    }
}
