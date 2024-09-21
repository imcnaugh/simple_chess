use crate::chess_piece::{ChessPiece, PieceType};
use crate::{Color, GameBoard};

/// # Game
///
/// Tracks a board game, typically for chess, but can be extended to modified versions of chess
pub struct Game {
    pub board: GameBoard,
    pub current_turn: Color,
    turn_number: u32,
    moves: Vec<String>,
}

impl Game {

    /// Creates a new where like chess, but can take a custom board
    pub fn new_game(board: GameBoard) -> Game {
        Game {
            board,
            current_turn: Color::White,
            turn_number: 1,
            moves: Vec::new(),
        }
    }

    /// Creates a chess game and builds a board with the pieces in the starting positions
    pub fn new_chess_game() -> Game {
        let board = Game::create_board_with_starting_position();

        Game {
            board,
            current_turn: Color::White,
            turn_number: 1,
            moves: Vec::new(),
        }
    }

    fn change_turn(&mut self) {
        self.current_turn = match self.current_turn {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }

    fn create_board_with_starting_position() -> GameBoard {
        let mut board = GameBoard::build_chess_board();

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
