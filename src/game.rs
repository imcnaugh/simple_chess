use crate::chess_move::ChessMoveType;
use crate::chess_move::ChessMoveType::Castle;
use crate::Color::{Black, White};
use crate::PieceType::{King, Pawn, Rook, Queen, Knight, Bishop};
use crate::{Color, GameBoard};
use crate::chess_board_square::SquareId;

/// # Game
///
/// Tracks a board game, typically for chess, but can be extended to modified versions of chess
pub struct Game {
    pub board: GameBoard,
    pub current_turn: Color,
    pub turn_number: u32,
    moves: Vec<ChessMoveType>,
    pub encoded_board_by_turn: Vec<Vec<u8>>,
    pub last_take_index: usize,
    fifty_move_rule_counter: usize,
    pub white_can_castle_short: bool,
    pub white_can_castle_long: bool,
    pub black_can_castle_short: bool,
    pub black_can_castle_long: bool,
}

impl Game {
    /// Creates a new chess game
    pub fn new_chess_game() -> Game {
        let starting_board = GameBoard::build_chess_board();
        let encoded_starting_position = starting_board.as_byte_arr();
        Game {
            board: starting_board,
            current_turn: White,
            turn_number: 1,
            moves: Vec::new(),
            encoded_board_by_turn: vec![encoded_starting_position],
            last_take_index: 0,
            fifty_move_rule_counter: 0,
            white_can_castle_short: true,
            white_can_castle_long: true,
            black_can_castle_short: true,
            black_can_castle_long: true,
        }
    }

    pub fn new_game(board: GameBoard, current_turn: Color) -> Self {
        let encoded_starting_position = board.as_byte_arr();
        Self {
            board,
            current_turn,
            turn_number: 1,
            moves: Vec::new(),
            encoded_board_by_turn: vec![encoded_starting_position],
            last_take_index: 0,
            fifty_move_rule_counter: 0,
            white_can_castle_short: true,
            white_can_castle_long: true,
            black_can_castle_short: true,
            black_can_castle_long: true,
        }
    }

    pub fn make_move(&mut self, m: &ChessMoveType) {
        if self.current_turn == Black {
            self.turn_number += 1;
        }

        self.update_50_move_rule_counter(m);
        self.update_can_can_castle(m);

        self.current_turn = match self.current_turn {
            White => Black,
            Black => White,
        };

        m.make_move(&mut self.board);

        self.add_new_board_position_to_game_history(m);

        self.moves.push(*m);
    }

    fn add_new_board_position_to_game_history(&mut self, m: &ChessMoveType) {
        let new_board_as_encoded = self.board.as_byte_arr();
        self.encoded_board_by_turn.push(new_board_as_encoded);
        if let ChessMoveType::Move { taken_piece: Some(_), .. } = m {
            self.last_take_index = self.encoded_board_by_turn.len() - 1;
        };
    }

    fn update_can_can_castle(&mut self, m: &ChessMoveType) {
        let castle_dir = match self.current_turn {
            White => (
                &mut self.white_can_castle_short,
                &mut self.white_can_castle_long,
            ),
            Black => (
                &mut self.black_can_castle_short,
                &mut self.black_can_castle_long,
            ),
        };

        if let Castle { .. } = m {
            *castle_dir.0 = false;
            *castle_dir.1 = false;
        }

        if let ChessMoveType::Move {
            piece,
            original_position,
            ..
        } = m
        {
            if piece.piece_type == King {
                *castle_dir.0 = false;
                *castle_dir.1 = false;
            }

            if piece.piece_type == Rook {
                if original_position.get_column() == 0 {
                    *castle_dir.1 = false;
                }

                if original_position.get_column() == self.board.get_width() - 1 {
                    *castle_dir.0 = false;
                }
            }
        }
    }

    fn update_50_move_rule_counter(&mut self, m: &ChessMoveType) {
        match m {
            ChessMoveType::EnPassant { .. } => self.fifty_move_rule_counter = 0,
            ChessMoveType::Move {
                taken_piece, piece, ..
            } => {
                if taken_piece.is_some() || piece.piece_type == Pawn {
                    self.fifty_move_rule_counter = 0
                } else {
                    self.fifty_move_rule_counter += 1
                }
            }
            _ => self.fifty_move_rule_counter += 1,
        };
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

    pub fn get_pgn(&self) -> String {
        let mut pgn = String::new();

        let mut switch = true;
        let mut turn = 1;

        for m in &self.moves {
            if switch {
               pgn.push_str(format!("{}. ", turn).as_str());
            }
            pgn.push_str(format!("{} ", m.get_standard_algebraic_notation()).as_str());
            if !switch {
                pgn.push_str("\n");
                turn+=1;
            }
            switch= !switch;
        }

        pgn
    }

    pub fn can_trigger_fifty_move_rule(&self) -> bool {
        self.fifty_move_rule_counter >= 100
    }
    
    pub fn can_trigger_draw_by_repetition(&self) -> bool {
        let current_board_as_encoded = self.encoded_board_by_turn.last();
        if current_board_as_encoded.is_none() {
            return false;
        }
        
        let mut repetition_count = 0;
        
        for index in self.last_take_index..self.encoded_board_by_turn.len() {
            let previous_board_as_encoded = &self.encoded_board_by_turn[index];
            
            if current_board_as_encoded.unwrap() == previous_board_as_encoded {
                repetition_count+=1;
            }
            
            if repetition_count >= 3 {
                return true;
            }
        }
        
        false
    }
    
    pub fn get_representation_as_FEN(&self) -> String {
        let ranks: Vec<String> = (0..self.board.get_height()).rev()
            .map(|rank| self.print_rank_as_FEN(rank))
            .collect();

        let joined_ranks = ranks.join("/");

        let current_turn_char = match self.current_turn {
            White => 'w',
            Black => 'b',
        };

        let castle_string = self.castle_string_as_FEN();

        let en_passant_string = self.en_passant_as_fen();

        let half_moves_clock = self.fifty_move_rule_counter;
        let turn_number = self.turn_number;

        format!("{joined_ranks} {current_turn_char} {castle_string} {en_passant_string} {half_moves_clock} {turn_number}")
    }

    fn en_passant_as_fen(&self) -> String {
        let (starting_row, en_passant_row, target_row) = match self.current_turn.opposite_color() {
            White => (1, 2, 3),
            Black => {
                let height = self.board.get_height();
                (height - 2, height - 3, height - 4)
            }
        };

        match self.moves.last() {
            Some(ChessMoveType::Move {piece, original_position, new_position, .. }) if piece.piece_type == Pawn => {
                if original_position.get_row() == starting_row && new_position .get_row() == target_row {
                    format!("{}", SquareId::build(original_position.get_column(), en_passant_row))
                } else {
                    "-".to_string()
                }
            },
            _ => "-".to_string()
        }
    }

    fn castle_string_as_FEN(&self) -> String {
        let mut castle_string = String::new();
        if self.white_can_castle_long {
            castle_string.push('Q');
        }
        if self.white_can_castle_short {
            castle_string.push('K');
        }
        if self.black_can_castle_long {
            castle_string.push('q');
        }
        if self.black_can_castle_short {
            castle_string.push('k');
        }

        if castle_string.is_empty() {
            castle_string.push('-');
        }
        castle_string
    }

    fn print_rank_as_FEN(&self, rank: usize) -> String {
        let mut resp = String::new();
        let mut empty_square_count = 0;
        for col in 0..self.board.get_width() {
            match self.board.check_space(col, rank) {
                None => empty_square_count+=1,
                Some(piece) => {
                    if empty_square_count != 0 {
                        resp.push_str(format!("{empty_square_count}").as_str());
                    }
                    let mut piece_char = match piece.piece_type {
                        Pawn => 'p',
                        Rook => 'r',
                        Knight => 'n',
                        Bishop => 'b',
                        Queen => 'q',
                        King => 'k'
                    };
                    if piece.color == White {
                        piece_char = piece_char.to_uppercase().last().unwrap();
                    }
                    resp.push(piece_char);
                    empty_square_count = 0;
                }
            }
        }
        if empty_square_count != 0 {
            resp.push_str(format!("{empty_square_count}").as_str());
        }
        resp
    }
}
