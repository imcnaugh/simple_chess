use game_board::Board;
use crate::Color;
use crate::piece::ChessPiece;

pub struct ChessGame {
    board: Board<dyn ChessPiece>,
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
            board: Board::<dyn ChessPiece>::build(8, 8).unwrap(),
            current_players_turn: Color::White,
            turn_number: 1,
            fifty_move_rule_counter: 0,
            can_white_castle_short: true,
            can_white_castle_long: true,
            can_black_castle_short: true,
            can_black_castle_long: true,
        }
    }

    /// Get board
    ///
    /// # Returns
    /// `Board<dyn ChessPiece>`: The board in its current state
    pub fn get_board(&self) -> &Board<dyn ChessPiece> {
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
        (self.can_white_castle_long,
        self.can_white_castle_short,
        self.can_black_castle_long,
        self.can_black_castle_short)
    }
}