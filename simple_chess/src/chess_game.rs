use crate::chess_game::DrawReason::{FiftyMoveRule, InsufficientPieces, Repetition};
use crate::chess_game_state_analyzer::{get_game_state, is_insufficient_material, GameState};
use crate::chess_move::ChessMoveType;
use crate::codec::binary::encode_board_as_binary;
use crate::piece::ChessPiece;
use crate::piece::PieceType::{Bishop, King, Knight, Pawn, Queen, Rook};
use crate::Color;
use crate::Color::{Black, White};
use game_board::Board;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct ChessGame {
    board: Board<ChessPiece>,
    current_players_turn: Color,
    turn_number: usize,
    fifty_move_rule_counter: usize,
    can_white_castle_short: bool,
    can_white_castle_long: bool,
    can_black_castle_short: bool,
    can_black_castle_long: bool,
    moves: Vec<ChessMoveType>,
    previous_board_states: Vec<Vec<u8>>,
}

#[derive(Debug)]
pub enum DrawReason {
    InsufficientPieces,
    Repetition,
    FiftyMoveRule,
}

fn build_board_with_starting_position() -> Board<ChessPiece> {
    let mut board = Board::<ChessPiece>::build(8, 8).unwrap();

    for (col, piece_type) in [Rook, Knight, Bishop, Queen, King, Bishop, Knight, Rook]
        .iter()
        .enumerate()
    {
        board.place_piece(ChessPiece::new(*piece_type, Black), col, 7);
        board.place_piece(ChessPiece::new(Pawn, Black), col, 6);
        board.place_piece(ChessPiece::new(Pawn, White), col, 1);
        board.place_piece(ChessPiece::new(*piece_type, White), col, 0);
    }

    board
}

impl ChessGame {
    /// Initialize a new simple_chess game.
    ///
    /// This function sets up a `ChessGame` with a starting board configuration,
    /// sets the current player's turn to white, initializes the turn number and
    /// fifty-move rule counter, and indicates that both sides may castle.
    ///
    /// # Returns
    ///
    /// `ChessGame`: A new instance of the `ChessGame` struct.
    ///
    /// # Examples
    ///
    /// ```
    /// use simple_chess::ChessGame;
    /// let game = ChessGame::new();
    /// ```
    pub fn new() -> ChessGame {
        ChessGame {
            board: build_board_with_starting_position(),
            current_players_turn: White,
            turn_number: 1,
            fifty_move_rule_counter: 0,
            can_white_castle_short: true,
            can_white_castle_long: true,
            can_black_castle_short: true,
            can_black_castle_long: true,
            moves: Vec::new(),
            previous_board_states: Vec::new(),
        }
    }

    pub fn build(
        board: Board<ChessPiece>,
        current_players_turn: Color,
        turn_number: usize,
        fifty_move_rule_counter: usize,
        can_white_castle_short: bool,
        can_white_castle_long: bool,
        can_black_castle_short: bool,
        can_black_castle_long: bool,
        moves: Vec<ChessMoveType>,
    ) -> ChessGame {
        ChessGame {
            board,
            current_players_turn,
            turn_number,
            fifty_move_rule_counter,
            can_white_castle_short,
            can_white_castle_long,
            can_black_castle_short,
            can_black_castle_long,
            moves,
            previous_board_states: vec![], // TODO generate previous board states from moves
        }
    }

    /// Get board
    ///
    /// # Returns
    /// `Board<ChessPiece>`: The board in its current state
    pub fn get_board(&self) -> &Board<ChessPiece> {
        &self.board
    }

    /// Get a mutable reference to the board
    ///
    /// This method provides mutable access to the simple_chess board, allowing for
    /// modifications to be made directly to the board's state. This can be useful
    /// when making moves or updating the board after certain actions during the game.
    ///
    /// # Returns
    ///
    /// `&mut Board<ChessPiece>`: A mutable reference to the board in its current state
    ///
    /// # Examples
    ///
    /// ```
    /// use simple_chess::ChessGame;
    /// let mut chess_game = ChessGame::new();
    /// let board = chess_game.get_board_mut();
    /// // Modify the board or make moves
    /// ```
    pub fn get_board_mut(&mut self) -> &mut Board<ChessPiece> {
        &mut self.board
    }

    /// Returns the color of the player whose turn it is.
    ///
    /// # Returns
    ///
    /// `Color`: An enum value representing the current player's turn.
    /// - This can be either `Color::White` or `Color::Black`.
    ///
    /// # Examples
    /// ```
    /// use simple_chess::{ChessGame, Color};
    /// let chess_game = ChessGame::new();
    /// // Assuming the game starts with White's turn
    /// assert_eq!(chess_game.get_current_players_turn(), Color::White);
    /// ```
    pub fn get_current_players_turn(&self) -> Color {
        self.current_players_turn
    }

    /// Get castling rights
    ///
    /// Returns a tuple containing four booleans that indicate the castling rights.
    /// The booleans represent:
    /// - (can_white_castle_long, can_white_castle_short, can_black_castle_long, can_black_castle_short)
    ///
    /// # Returns
    ///
    /// (bool, bool, bool, bool): A tuple representing the castling rights for white and black players.
    pub fn get_castling_rights(&self) -> (bool, bool, bool, bool) {
        (
            self.can_white_castle_long,
            self.can_white_castle_short,
            self.can_black_castle_long,
            self.can_black_castle_short,
        )
    }

    /// Returns the current turn number.
    ///
    /// # Returns
    ///
    /// `usize`: The current turn number in the game.
    /// This method returns the total number of turns that have been taken in the game so far.
    ///
    /// # Examples
    ///
    /// ```
    /// use simple_chess::ChessGame;
    /// let chess_game = ChessGame::new();
    /// assert_eq!(chess_game.get_turn_number(), 1);
    /// ```
    pub fn get_turn_number(&self) -> usize {
        self.turn_number
    }

    /// Get the list of moves made so far.
    ///
    /// # Returns
    ///
    /// `&Vec<ChessMoveType>`: A reference to the vector containing all the moves made in the game so far.
    ///
    /// # Examples
    ///
    /// ```
    /// use simple_chess::{ChessGame, ChessMoveType};
    /// let chess_game = ChessGame::new();
    /// // Assuming no moves have been made yet
    /// assert!(chess_game.get_moves().is_empty());
    /// ```
    pub fn get_moves(&self) -> &Vec<ChessMoveType> {
        &self.moves
    }

    /// Get the last move made in the game.
    ///
    /// # Returns
    ///
    /// `Option<&ChessMoveType>`: An optional reference to the last move made.
    /// If no moves have been made, this method returns `None`.
    ///
    /// # Examples
    ///
    /// ```
    /// use simple_chess::{ChessGame, ChessMoveType};
    /// let chess_game = ChessGame::new();
    /// // Assuming no moves have been made yet
    /// assert_eq!(chess_game.get_last_move(), None);
    /// ```
    pub fn get_last_move(&self) -> Option<&ChessMoveType> {
        self.moves.last()
    }

    /// Get the fifty-move rule counter
    ///
    /// # Returns
    ///
    /// `usize`: The current count of half-moves since the last capture or pawn move.
    /// This counter is used to determine if the fifty-move rule has been reached,
    /// which allows a player to claim a draw if fifty consecutive moves have been
    /// made without any pawn movement or piece capture.
    ///
    /// # Examples
    ///
    /// ```
    /// use simple_chess::ChessGame;
    /// let chess_game = ChessGame::new();
    /// assert_eq!(chess_game.get_50_move_rule_counter(), 0);
    /// ```
    pub fn get_50_move_rule_counter(&self) -> usize {
        self.fifty_move_rule_counter
    }

    /// Executes a given move on the simple_chess board.
    ///
    /// # Arguments
    ///
    /// * `chess_move` - An instance of `ChessMoveType` representing the move to be made on the board.
    ///
    /// # Effects
    ///
    /// - The move is applied to the internal board representation.
    /// - The turn number is incremented if it was Black's turn.
    /// - Updates internal state for castling rights and the fifty-move rule counter.
    /// - Alternates the current player's turn.
    /// - Adds the move to the move history and updates previous board states.
    pub fn make_move(&mut self, chess_move: ChessMoveType) {
        chess_move.make_move(&mut self.board);
        if self.current_players_turn == Black {
            self.turn_number += 1;
        }

        match chess_move {
            ChessMoveType::Move {
                taken_piece,
                piece,
                original_position,
                new_position,
                ..
            } => {
                if taken_piece.is_some() || piece.get_piece_type() == Pawn {
                    self.fifty_move_rule_counter = 0;
                    self.previous_board_states = vec![];
                } else {
                    self.fifty_move_rule_counter += 1;
                }

                self.update_castling_rights(taken_piece, piece, original_position, new_position);
            }
            ChessMoveType::Castle { .. } => {
                match self.current_players_turn {
                    White => {
                        self.can_white_castle_short = false;
                        self.can_white_castle_long = false;
                    }
                    Black => {
                        self.can_black_castle_short = false;
                        self.can_black_castle_long = false;
                    }
                }
                self.fifty_move_rule_counter = 0;
            }
            _ => {
                self.fifty_move_rule_counter = 0;
                self.previous_board_states = vec![];
            }
        }

        self.moves.push(chess_move);
        self.previous_board_states
            .push(encode_board_as_binary(self.get_board()));
        self.current_players_turn = self.current_players_turn.opposite();
    }


    /// Undoes the last move made in the game.
    ///
    /// # Effects
    ///
    /// - If no moves have been made, the method simply returns without making any changes.
    /// - If there is a move to undo:
    ///   - The move is removed from the move history.
    ///   - The effects of the last move are reverted on the board.
    ///
    /// This method can be used to revert a move in case of user mistakes or for implementing
    /// a "takeback" feature in the game.
    ///
    /// # Examples
    ///
    /// ```
    /// use simple_chess::ChessGame;
    /// use simple_chess::ChessMoveType;
    /// use simple_chess::Color::White;
    /// use simple_chess::piece::ChessPiece;
    /// use simple_chess::piece::PieceType::Pawn;
    /// let mut chess_game = ChessGame::new();
    ///
    /// let chess_move = ChessMoveType::Move {
    ///     taken_piece: None,
    ///     piece: ChessPiece::new(Pawn, White),
    ///     original_position: (1, 0),
    ///     new_position: (2, 0),
    ///     promotion: None,
    /// };
    ///
    /// chess_game.make_move(chess_move);
    /// assert_eq!(chess_game.get_last_move().is_some(), true);
    ///
    /// chess_game.undo_last_move();
    /// assert_eq!(chess_game.get_last_move(), None);
    /// ```
    pub fn undo_last_move(&mut self) {
        if self.moves.is_empty() {
            return;
        }
        let last_move = self.moves.pop().unwrap();
        last_move.undo_move(&mut self.board);
    }

    fn update_castling_rights(
        &mut self,
        taken_piece: Option<ChessPiece>,
        piece: ChessPiece,
        original_position: (usize, usize),
        new_position: (usize, usize),
    ) {
        if let Some(piece) = taken_piece {
            if piece.get_piece_type() == Rook {
                match piece.get_color() {
                    White => {
                        if new_position.1 == 0 {
                            if new_position.0 == 0 {
                                self.can_white_castle_long = false;
                            }
                            if new_position.0 == self.board.get_width() - 1 {
                                self.can_white_castle_short = false;
                            }
                        }
                    }
                    Black => {
                        if new_position.1 == self.board.get_height() - 1 {
                            if new_position.0 == 0 {
                                self.can_black_castle_long = false;
                            }
                            if new_position.0 == self.board.get_width() - 1 {
                                self.can_black_castle_short = false;
                            }
                        }
                    }
                }
            }
        }

        if piece.get_piece_type() == Rook {
            if original_position.0 < self.board.get_width() / 2 {
                match self.current_players_turn {
                    White => self.can_white_castle_long = false,
                    Black => self.can_black_castle_long = false,
                }
            } else {
                match self.current_players_turn {
                    White => self.can_white_castle_short = false,
                    Black => self.can_black_castle_short = false,
                }
            }
        }

        if piece.get_piece_type() == King {
            match self.current_players_turn {
                White => {
                    self.can_white_castle_long = false;
                    self.can_white_castle_short = false;
                }
                Black => {
                    self.can_black_castle_long = false;
                    self.can_black_castle_short = false;
                }
            }
        }
    }

    /// Get the current state of the game.
    ///
    /// # Returns
    ///
    /// `GameState`: The current state of the game, which can be calculated
    /// based on various factors like board configuration, move history, etc.
    ///
    /// This method internally calls a function to determine the game state
    /// and returns the result.
    ///
    pub fn get_game_state(&mut self) -> GameState {
        get_game_state(self)
    }

    ///
    /// Determines if a draw can be claimed in the game based on specific rules.
    ///
    /// # Returns
    ///
    /// `Option<DrawReason>`: An optional `DrawReason` indicating the reason a draw can be claimed.
    /// Returns `None` if a draw cannot be claimed.
    ///
    /// A draw can be claimed based on:
    ///
    /// - The fifty-move rule: If fifty moves have been made without a pawn move or piece capture.
    /// - Insufficient material: If the material left on the board is not enough for a checkmate.
    /// - Repetition: If the same board state has been repeated three times.
    ///
    ///
    pub fn can_claim_draw(&self) -> Option<DrawReason> {
        if self.fifty_move_rule_counter >= 100 {
            return Some(FiftyMoveRule);
        }
        if is_insufficient_material(self.get_board()) {
            return Some(InsufficientPieces);
        }
        if self.can_claim_draw_by_repetition() {
            return Some(Repetition);
        }
        None
    }

    fn can_claim_draw_by_repetition(&self) -> bool {
        let mut previous_board_states: HashMap<Vec<u8>, usize> = HashMap::new();
        for previous_state in &self.previous_board_states {
            match previous_board_states.get(previous_state) {
                None => {
                    previous_board_states.insert(previous_state.clone(), 1);
                }
                Some(count) => {
                    if *count > 2 {
                        return true;
                    }
                    let new_count = count + 1;
                    previous_board_states.insert(previous_state.clone(), new_count);
                }
            }
        }
        false
    }
}

impl Default for ChessGame {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::codec::forsyth_edwards_notation::encode_game_as_string;

    #[test]
    fn new_game_start_correctly() {
        let game = ChessGame::new();
        let fen_string = encode_game_as_string(&game);
        assert_eq!(
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
            fen_string
        );
    }
}
