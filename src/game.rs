use crate::chess_move::{ChessMoveType};
use crate::Color::{Black, White};
use crate::{ChessPiece, Color, GameBoard};
use crate::PieceType::{Pawn, Queen};

/// # Game
///
/// Tracks a board game, typically for chess, but can be extended to modified versions of chess
pub struct Game {
    pub board: GameBoard,
    pub current_turn: Color,
    pub turn_number: u32,
    moves: Vec<ChessMoveType>,
    fifty_move_rule_counter: usize,
    pub white_can_castle_short: bool,
    pub white_can_castle_long: bool,
    pub black_can_castle_short: bool,
    pub black_can_castle_long: bool,
}

impl Game {
    /// Creates a new chess game
    pub fn new_chess_game() -> Game {
        Game {
            board: GameBoard::build_chess_board(),
            current_turn: Color::White,
            turn_number: 1,
            moves: Vec::new(),
            fifty_move_rule_counter: 0,
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
            fifty_move_rule_counter: 0,
            white_can_castle_short: true,
            white_can_castle_long: true,
            black_can_castle_short: true,
            black_can_castle_long: true,
        }
    }

    pub fn change_turn(&mut self, m: ChessMoveType) {
        if self.current_turn == Black {
            self.turn_number += 1;
        }

        match m {
            ChessMoveType::EnPassant {..} => self.fifty_move_rule_counter = 0,
            ChessMoveType::Move {taken_piece, ..} => {
                if taken_piece.is_some() {
                    self.fifty_move_rule_counter = 0
                } else {
                    self.fifty_move_rule_counter += 1
                }
            },
            ChessMoveType::Castle {..} => self.fifty_move_rule_counter += 1,
        };

        self.current_turn = match self.current_turn {
            White => Black,
            Black => White,
        };

        m.make_move(&mut self.board);

        self.moves.push(m);
    }

    pub fn get_board_mut(&mut self) -> &mut GameBoard {
        &mut self.board
    }

    pub fn get_board(&self) -> &GameBoard {
        &self.board
    }

    pub fn get_moves(&self) -> &Vec<ChessMoveType> {
        &self.moves
    }

    pub fn can_trigger_fifty_move_rule(&self) -> bool {
        self.fifty_move_rule_counter >= 100
    }
}
