use crate::chess_move::ChessMoveType;
use crate::piece::ChessPiece;
use crate::{ChessGame, Color};
use game_board::Board;

/// The `ChessGameBuilder` struct is used to construct a `ChessGame`
/// instance. It employs the builder pattern to set up various
/// aspects of the game, ensuring that all necessary fields are
/// configured before the game can commence.
///
/// # Required Fields
/// - `board`: The simple_chess board configuration.
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
    moves: Option<Vec<ChessMoveType>>,
}

impl ChessGameBuilder {
    /// Creates a new instance of `ChessGameBuilder` with all fields
    /// set to `None`, representing an uninitialized state.
    ///
    /// # Examples
    ///
    /// ```
    /// use simple_chess::ChessGameBuilder;
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
            moves: None,
        }
    }

    /// Finalizes the construction of a `ChessGame` instance.
    ///
    /// This method checks if the necessary components (board and current player's turn)
    /// for a simple_chess game are set. If any of these components are missing, it returns an
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
    /// use simple_chess::{ChessGameBuilder, Color, ChessMoveType};
    /// use simple_chess::Color::White;
    /// use simple_chess::piece::ChessPiece;
    /// use simple_chess::piece::PieceType::Pawn;
    /// use game_board::Board;
    ///
    /// let board: Board<ChessPiece> = Board::build(8, 8).unwrap();
    ///
    /// let moves: Vec<ChessMoveType> = vec![ChessMoveType::Move {
    ///     original_position: (3, 1),
    ///     new_position: (3, 3),
    ///     piece: ChessPiece::new(Pawn, White),
    ///     taken_piece: None,
    ///     promotion: None
    /// }];
    ///
    /// let game_result = ChessGameBuilder::new()
    ///     .set_board(board)
    ///     .set_current_turn(White)
    ///     .set_turn_number(1)
    ///     .set_fifty_move_rule_counter(0)
    ///     .set_castle_rights(true, true, true, true)
    ///     .set_moves(moves)
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
                self.moves.unwrap_or_default(),
            );
            Ok(game)
        } else {
            Err("Not all necessary elements are set")
        }
    }

    /// Sets the board for the `ChessGame`.
    ///
    /// This method allows you to set the simple_chess board configuration to be used in the game.
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
    /// counter. The fifty-move rule in simple_chess states that a player can claim a draw
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

    /// Sets the moves made so far in the `ChessGame`.
    ///
    /// This method allows you to specify the sequence of moves that have been made
    /// so far in the game. Each move is represented by a `ChessMoveType`.
    ///
    /// # Arguments
    ///
    /// * `moves` - A `Vec<ChessMoveType>` containing the moves made in the game.
    ///
    /// # Returns
    ///
    /// * `Self` - Returns the `ChessGameBuilder` instance with the moves set.
    pub fn set_moves(mut self, moves: Vec<ChessMoveType>) -> Self {
        self.moves = Some(moves);
        self
    }
}

impl Default for ChessGameBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::piece::PieceType::Pawn;
    use crate::Color::{Black, White};

    #[test]
    fn build_basic_game_with_defaults() {
        let mut builder = ChessGameBuilder::new();
        let board = Board::<ChessPiece>::build(8, 8).unwrap();

        builder = builder.set_board(board);
        builder = builder.set_current_turn(White);

        let game_result = builder.build();
        assert!(game_result.is_ok());

        let game_result = game_result.unwrap();

        assert_eq!(8, game_result.get_board().get_width());
        assert_eq!(8, game_result.get_board().get_height());
        assert_eq!(White, game_result.get_current_players_turn());
        assert_eq!(0, game_result.get_50_move_rule_counter());
        assert_eq!(0, game_result.get_turn_number());
        let (castle_wl, castle_ws, castle_bl, castle_bs) = game_result.get_castling_rights();
        assert!(castle_wl);
        assert!(castle_ws);
        assert!(castle_bl);
        assert!(castle_bs);
    }

    #[test]
    fn build_game() {
        let mut builder = ChessGameBuilder::new();
        let board = Board::<ChessPiece>::build(2, 5).unwrap();
        let moves: Vec<ChessMoveType> = vec![
            ChessMoveType::Move {
                original_position: (3, 1),
                new_position: (3, 3),
                piece: ChessPiece::new(Pawn, White),
                taken_piece: None,
                promotion: None,
            },
            ChessMoveType::Move {
                original_position: (3, 6),
                new_position: (3, 4),
                piece: ChessPiece::new(Pawn, Black),
                taken_piece: None,
                promotion: None,
            },
        ];

        builder = builder.set_board(board);
        builder = builder.set_current_turn(Color::Black);
        builder = builder.set_castle_rights(false, true, false, true);
        builder = builder.set_turn_number(9);
        builder = builder.set_fifty_move_rule_counter(800);
        builder = builder.set_moves(moves);

        let game_result = builder.build();
        assert!(game_result.is_ok());

        let game_result = game_result.unwrap();

        assert_eq!(2, game_result.get_board().get_width());
        assert_eq!(5, game_result.get_board().get_height());
        assert_eq!(Color::Black, game_result.get_current_players_turn());
        assert_eq!(800, game_result.get_50_move_rule_counter());
        assert_eq!(9, game_result.get_turn_number());
        let (castle_wl, castle_ws, castle_bl, castle_bs) = game_result.get_castling_rights();
        assert!(castle_wl);
        assert!(!castle_ws);
        assert!(castle_bl);
        assert!(!castle_bs);

        assert_eq!(2, game_result.get_moves().len());
    }
}
