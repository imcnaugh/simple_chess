use crate::piece::ChessPiece;
use crate::Color;
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
            // TODO setup the board in starting position
            board: Board::<ChessPiece>::build(8, 8).unwrap(),
            current_players_turn: Color::White,
            turn_number: 1,
            fifty_move_rule_counter: 0,
            can_white_castle_short: true,
            can_white_castle_long: true,
            can_black_castle_short: true,
            can_black_castle_long: true,
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
        }
    }

    /// Get board
    ///
    /// # Returns
    /// `Board<dyn ChessPiece>`: The board in its current state
    pub fn get_board(&self) -> &Board<ChessPiece> {
        &self.board
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
}
