use crate::piece::ChessPiece;
use crate::{ChessGame, Color};
use game_board::Board;

/// The `ChessGameBuilder` struct is used to construct a `ChessGame`
/// instance. It employs the builder pattern to set up various
/// aspects of the game, ensuring that all necessary fields are
/// configured before the game can commence.
///
/// # Required Fields
/// - `board`: The chess board configuration.
/// - `current_players_turn`: The color of the player whose turn it is.
pub struct ChessGameBuilder {
    board: Option<Board<ChessPiece>>,
    current_players_turn: Option<Color>,
    turn_number: Option<usize>,
    fifty_move_rule_counter: Option<usize>,
    can_white_castle_short: Option<bool>,
    can_white_castle_long: Option<bool>,
    can_black_castle_short: Option<bool>,
    can_black_castle_long: Option<bool>,
}

impl ChessGameBuilder {
    /// Creates a new instance of `ChessGameBuilder` with all fields
    /// set to `None`, representing an uninitialized state.
    ///
    /// # Examples
    ///
    /// ```
    /// use chess_game::ChessGameBuilder;
    ///
    /// let builder = ChessGameBuilder::new();
    /// // Now you can set various fields using the builder methods.
    /// ```
    pub fn new() -> Self {
        Self {
            board: None,
            current_players_turn: None,
            turn_number: None,
            fifty_move_rule_counter: None,
            can_white_castle_short: None,
            can_white_castle_long: None,
            can_black_castle_short: None,
            can_black_castle_long: None,
        }
    }

    /// Finalizes the construction of a `ChessGame` instance.
    ///
    /// This method checks if the necessary components (board and current player's turn)
    /// for a chess game are set. If any of these components are missing, it returns an
    /// error; otherwise, it constructs and returns a `ChessGame` instance.
    ///
    /// # Returns
    ///
    /// * `Ok(ChessGame)` - If the necessary components (board and current player's turn) are set
    /// * `Err(&str)` - If any of the necessary components is missing
    ///
    /// # Examples
    ///
    /// ```
    /// use chess_game::{ChessGameBuilder, Color};
    /// use chess_game::piece::ChessPiece;
    /// use game_board::Board;
    ///
    /// let board: Board<ChessPiece> = Board::build(8, 8).unwrap();
    ///
    /// let game_result = ChessGameBuilder::new()
    ///     .set_board(board)
    ///     .set_current_turn(Color::White)
    ///     .set_turn_number(1)
    ///     .set_fifty_move_rule_counter(0)
    ///     .set_castle_rights(true, true, true, true)
    ///     .build();
    ///
    /// assert!(game_result.is_ok());
    /// ```
    pub fn build<'a>(self) -> Result<ChessGame, &'a str> {
        if let (Some(board), Some(current_players_turn)) = (self.board, self.current_players_turn) {
            let game = ChessGame::build(
                board,
                current_players_turn,
                self.turn_number.unwrap_or(0),
                self.fifty_move_rule_counter.unwrap_or(0),
                self.can_white_castle_short.unwrap_or(true),
                self.can_white_castle_long.unwrap_or(true),
                self.can_black_castle_short.unwrap_or(true),
                self.can_black_castle_long.unwrap_or(true),
            );
            Ok(game)
        } else {
            Err("Not all necessary elements are set")
        }
    }

    /// Sets the board for the `ChessGame`.
    ///
    /// This method allows you to set the chess board configuration to be used in the game.
    ///
    /// # Arguments
    ///
    /// * `board` - A `Board` instance containing `ChessPiece` configurations.
    ///
    /// # Returns
    ///
    /// * `Self` - Returns the `ChessGameBuilder` instance with the board set.
    pub fn set_board(mut self, board: Board<ChessPiece>) -> Self {
        self.board = Some(board);
        self
    }

    /// Sets the current player's turn for the `ChessGame`.
    ///
    /// This method allows you to specify which player (color) will take the next turn.
    ///
    /// # Arguments
    ///
    /// * `color` - A `Color` value representing the player whose turn it is.
    ///
    /// # Returns
    ///
    /// * `Self` - Returns the `ChessGameBuilder` instance with the current player's turn set.
    pub fn set_current_turn(mut self, color: Color) -> Self {
        self.current_players_turn = Some(color);
        self
    }

    /// Sets the turn number for the `ChessGame`.
    ///
    /// This method allows you to specify the turn number at which the game starts.
    ///
    /// # Arguments
    ///
    /// * `turn_number` - A `usize` value representing the starting turn number.
    ///
    /// # Returns
    ///
    /// * `Self` - Returns the `ChessGameBuilder` instance with the turn number set.
    pub fn set_turn_number(mut self, turn_number: usize) -> Self {
        self.turn_number = Some(turn_number);
        self
    }

    /// Sets the counter for the fifty-move rule in the `ChessGame`.
    ///
    /// This method allows you to specify the current state of the fifty-move rule
    /// counter. The fifty-move rule in chess states that a player can claim a draw
    /// if no pawn has been moved and no capture has been made in the last fifty moves.
    ///
    /// # Arguments
    ///
    /// * `fifty_move_rule_counter` - A `usize` value representing the number of
    ///   moves that have been made without any pawn movement or capture.
    ///
    /// # Returns
    ///
    /// * `Self` - Returns the `ChessGameBuilder` instance with the fifty-move rule
    ///   counter set.
    pub fn set_fifty_move_rule_counter(mut self, fifty_move_rule_counter: usize) -> Self {
        self.fifty_move_rule_counter = Some(fifty_move_rule_counter);
        self
    }

    /// Sets the castling rights for both players in the `ChessGame`.
    ///
    /// This method allows you to specify whether each player can castle short or long.
    ///
    /// # Arguments
    ///
    /// * `ws` - A `bool` value indicating if White can castle short.
    /// * `wl` - A `bool` value indicating if White can castle long.
    /// * `bs` - A `bool` value indicating if Black can castle short.
    /// * `bl` - A `bool` value indicating if Black can castle long.
    ///
    /// # Returns
    ///
    /// * `Self` - Returns the `ChessGameBuilder` instance with the castling rights set.
    pub fn set_castle_rights(mut self, ws: bool, wl: bool, bs: bool, bl: bool) -> Self {
        self.can_white_castle_short = Some(ws);
        self.can_white_castle_long = Some(wl);
        self.can_black_castle_short = Some(bs);
        self.can_black_castle_long = Some(bl);
        self
    }
}
