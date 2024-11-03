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
    /// Creates a new chess game
    pub fn new_game_chess() -> Game {
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
    
    fn change_turn(&mut self) {
        self.current_turn = match self.current_turn {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
}
