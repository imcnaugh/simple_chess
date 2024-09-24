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
    white_can_castle_short: bool,
    white_can_castle_long: bool,
    black_can_castle_short: bool,
    black_can_castle_long: bool,
}

impl Game {
    /// Creates a new where like chess, but can take a custom board
    pub fn new_game(board: GameBoard) -> Game {
        Game {
            board,
            current_turn: Color::White,
            turn_number: 1,
            moves: Vec::new(),
            white_can_castle_short: true,
            white_can_castle_long: true,
            black_can_castle_short: true,
            black_can_castle_long: true,
        }
    }

    /// Creates a chess game and builds a board with the pieces in the starting positions
    pub fn new_chess_game() -> Game {
        Game::new_game(Game::create_board_with_starting_position())
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
            board.place_piece(ChessPiece::new(Color::White, PieceType::Pawn), i, 1);
            board.place_piece(ChessPiece::new(Color::Black, PieceType::Pawn), i, 6);
        }

        board.place_piece(ChessPiece::new(Color::White, PieceType::Rook), 0, 0);
        board.place_piece(ChessPiece::new(Color::White, PieceType::Rook), 7, 0);
        board.place_piece(ChessPiece::new(Color::Black, PieceType::Rook), 0, 7);
        board.place_piece(ChessPiece::new(Color::Black, PieceType::Rook), 7, 7);
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
        board.place_piece(ChessPiece::new(Color::White, PieceType::King), 4, 0);
        board.place_piece(ChessPiece::new(Color::Black, PieceType::King), 4, 7);

        board
    }
}
