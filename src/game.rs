use crate::chess_piece::{ChessPiece, PieceType};
use crate::{Color, GameBoard};
use crate::chess_move::ChessMove;

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
    /// Creates a new chess game
    pub fn new_chess_game() -> Game {
        Game {
            board: GameBoard::build_chess_board(),
            current_turn: Color::White,
            turn_number: 1,
            moves: Vec::new(),
            white_can_castle_short: true,
            white_can_castle_long: true,
            black_can_castle_short: true,
            black_can_castle_long: true,
        }
    }

    pub fn new_game(board: GameBoard, current_turn: Color) -> Self {
        Self {
            board,
            current_turn,
            turn_number: 1,
            moves: Vec::new(),
            white_can_castle_short: true,
            white_can_castle_long: true,
            black_can_castle_short: true,
            black_can_castle_long: true,
        }
    }

    fn change_turn(&mut self) {
        self.current_turn = match self.current_turn {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }

    pub fn get_board_mut(&mut self) -> &mut GameBoard {
        &mut self.board
    }
    
    pub fn get_board(&self) -> &GameBoard {
        &self.board
    }
    
    pub fn make_move(&mut self, chess_move: &ChessMove) {
        self.board.place_piece(*chess_move.piece, chess_move.new_position.0, chess_move.new_position.1);
        self.board.remove_piece(chess_move.original_position.0, chess_move.original_position.1);
    }
}
