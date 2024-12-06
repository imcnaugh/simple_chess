use crate::chess_game_state_analyzer::{get_game_state, GameState};
use crate::chess_move::ChessMoveType;
use crate::piece::PieceType::{Bishop, King, Knight, Pawn, Queen, Rook};
use crate::piece::ChessPiece;
use crate::Color;
use crate::Color::{Black, White};
use game_board::Board;

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
    /// Initialize a new chess game.
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
    /// use chess_game::ChessGame;
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
    /// This method provides mutable access to the chess board, allowing for
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
    /// use chess_game::ChessGame;
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
    /// use chess_game::{ChessGame, Color};
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
    /// use chess_game::ChessGame;
    /// let chess_game = ChessGame::new();
    /// assert_eq!(chess_game.get_50_move_rule_counter(), 0);
    /// ```
    pub fn get_50_move_rule_counter(&self) -> usize {
        self.fifty_move_rule_counter
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
    /// use chess_game::ChessGame;
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
    /// use chess_game::{ChessGame, ChessMoveType};
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
    /// use chess_game::{ChessGame, ChessMoveType};
    /// let chess_game = ChessGame::new();
    /// // Assuming no moves have been made yet
    /// assert_eq!(chess_game.get_last_move(), None);
    /// ```
    pub fn get_last_move(&self) -> Option<&ChessMoveType> {
        self.moves.last()
    }

    pub fn make_move(&mut self, chess_move: ChessMoveType) -> GameState {
        chess_move.make_move(&mut self.board);
        if self.current_players_turn == Black {
            self.turn_number += 1;
        }

        match chess_move {
            ChessMoveType::Move {
                taken_piece,
                piece,
                original_position,
                ..
            } => {
                if taken_piece.is_some() || piece.get_piece_type() == Pawn {
                    self.fifty_move_rule_counter = 0;
                } else {
                    self.fifty_move_rule_counter += 1;
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
            }
        }

        self.current_players_turn = self.current_players_turn.opposite();
        self.moves.push(chess_move);

        self.get_game_state()
    }

    pub fn get_game_state(&mut self) -> GameState {
        get_game_state(self)
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
