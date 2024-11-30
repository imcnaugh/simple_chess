use crate::chess_game::ChessGame;
use crate::chess_game_builder::ChessGameBuilder;
use crate::piece::{ChessPiece, PieceType};
use crate::ChessMoveType;
use crate::ChessMoveType::EnPassant;
use crate::Color::{Black, White};
use game_board::Board;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

/// Encodes the current state of the chess game as a string in FEN (Forsyth-Edwards Notation) format.
///
/// The resulting string consists of the following parts:
///
/// 1. The board layout, represented by rows separated by slashes, where each piece is represented
///    by a character and empty squares are represented by numbers.
/// 2. The current turn, indicated by 'w' for White or 'b' for Black.
/// 3. Castling rights, represented by 'K', 'Q', 'k', and 'q' for White king-side, White queen-side,
///    Black king-side, and Black queen-side castling respectively. If no castling rights are available,
///    a dash '-' is used instead.
/// 4. The en passant target square, represented by the algebraic notation of the target square
///    for en passant capture, such as 'e3'. If no en passant target square is available, a dash '-'
///    is used instead.
/// 5. The number of half-moves since the last capture or pawn advance, for the fifty-move rule.
/// 6. The full move number, starting from 1 and incremented after Black's turn.
///
/// # Arguments
///
/// * `game` - A reference to the `ChessGame` instance representing the current state of the game.
///
/// # Returns
///
/// A `String` representing the current state of the chess game.
pub fn encode_game_as_string(game: &ChessGame) -> String {
    format!(
        "{} {} {} {} {} {}",
        get_board_as_fen_string(game),
        get_current_turn_char(game),
        get_castling_rights(game),
        get_en_passent(game),
        game.get_50_move_rule_counter(),
        game.get_turn_number()
    )
}

/// Builds a `ChessGame` from a string in Forsyth-Edwards Notation (FEN) format.
///
/// This function parses the FEN string and constructs the game state, including the board layout,
/// current turn, castling rights, en passant target square, half-move counter, and full move number.
///
/// # Arguments
///
/// * `fen_string` - A string slice representing the state of the chess game in FEN format.
///
/// # Returns
///
/// A `Result` which is `Ok` if the `ChessGame` was built successfully, or an `Err` containing
/// a `ForsythEdwardsNotationError` if the FEN string is invalid or cannot be parsed.
pub fn build_game_from_string(fen_string: &str) -> Result<ChessGame, ForsythEdwardsNotationError> {
    let fen_string = fen_string.trim();
    if fen_string.is_empty() {
        return Err(ForsythEdwardsNotationError::new(
            "argument must be a string in Forsyth–Edwards Notation".to_string(),
        ));
    }

    let steps = [
        parse_board_from_string,
        parse_current_turn_from_string,
        parse_castling_rights_from_string,
        parse_en_passant_option_from_string,
        parse_half_turn_counter_from_string,
        parse_turn_number_from_string,
    ];

    let mut parts = fen_string.split(" ");
    let mut builder = ChessGameBuilder::new();

    for step in steps {
        if let Some(next) = parts.next() {
            builder = match step(builder, next) {
                Ok(g) => g,
                Err(e) => return Err(e),
            };
        } else {
            return Err(ForsythEdwardsNotationError::new(
                "Missing some parts of the string".to_string(),
            ));
        }
    }

    match builder.build() {
        Ok(g) => Ok(g),
        Err(e) => Err(ForsythEdwardsNotationError::new(e.to_string())),
    }
}

fn parse_board_from_string(
    builder: ChessGameBuilder,
    board_as_fen_string: &str,
) -> Result<ChessGameBuilder, ForsythEdwardsNotationError> {
    let mut board = Board::build(8, 8).unwrap();

    let mut files = board_as_fen_string.split("/");

    let mut col = 0;
    for row in (0..8).rev() {
        let file = files.next().unwrap();

        for c in file.chars() {
            match c {
                '1'..='8' => {
                    col += c.to_digit(10).unwrap() as usize;
                }
                _ => {
                    let piece = match c {
                        'P' => ChessPiece::new(PieceType::Pawn, White),
                        'p' => ChessPiece::new(PieceType::Pawn, Black),
                        'R' => ChessPiece::new(PieceType::Rook, White),
                        'r' => ChessPiece::new(PieceType::Rook, Black),
                        'N' => ChessPiece::new(PieceType::Knight, White),
                        'n' => ChessPiece::new(PieceType::Knight, Black),
                        'B' => ChessPiece::new(PieceType::Bishop, White),
                        'b' => ChessPiece::new(PieceType::Bishop, Black),
                        'Q' => ChessPiece::new(PieceType::Queen, White),
                        'q' => ChessPiece::new(PieceType::Queen, Black),
                        'K' => ChessPiece::new(PieceType::King, White),
                        'k' => ChessPiece::new(PieceType::King, Black),
                        _ => {
                            return Err(ForsythEdwardsNotationError::new(format!(
                                "Unexpected char '{c}' in file '{file}' of piece placement data"
                            )))
                        }
                    };
                    board.place_piece(piece, col, row);
                    col += 1;
                }
            }
        }

        if col != 8 {
            return Err(ForsythEdwardsNotationError::new(format!(
                "File '{file}' was not 8 spaces long in piece placement data"
            )));
        }

        col = 0;
    }

    Ok(builder.set_board(board))
}

fn parse_current_turn_from_string(
    builder: ChessGameBuilder,
    current_turn_string: &str,
) -> Result<ChessGameBuilder, ForsythEdwardsNotationError> {
    match current_turn_string {
        "w" => Ok(builder.set_current_turn(White)),
        "b" => Ok(builder.set_current_turn(Black)),
        _ => Err(ForsythEdwardsNotationError::new(format!("encountered unexpected token parsing turn from FEN string, Expected 'w' or 'b', received {current_turn_string}")))
    }
}

fn parse_castling_rights_from_string(
    builder: ChessGameBuilder,
    castling_rights_string: &str,
) -> Result<ChessGameBuilder, ForsythEdwardsNotationError> {
    let (mut ws, mut wl, mut bs, mut bl) = (false, false, false, false);
    if castling_rights_string != "-" {
        for c in castling_rights_string.chars() {
            match c {
                'K' => ws = true,
                'Q' => wl = true,
                'k' => bs = true,
                'q' => bl = true,
                _ => {
                    return Err(ForsythEdwardsNotationError::new(format!(
                        "Unexpected char '{c}' in castling rights string"
                    )))
                }
            }
        }
    }

    Ok(builder.set_castle_rights(ws, wl, bs, bl))
}

fn parse_en_passant_option_from_string(
    builder: ChessGameBuilder,
    en_passent_option_string: &str,
) -> Result<ChessGameBuilder, ForsythEdwardsNotationError> {
    if en_passent_option_string == "-" {
        Ok(builder)
    } else {
        match game_board::get_column_and_row_from_square_name(en_passent_option_string) {
            Ok((col, row)) => {
                let pawn_color = if row < 3 { White } else { Black };
                let (original_row, new_row) = match pawn_color {
                    White => (row - 1, row + 1),
                    Black => (row + 1, row - 1),
                };

                let m = ChessMoveType::Move {
                    original_position: (col, original_row),
                    new_position: (col, new_row),
                    piece: ChessPiece::new(PieceType::Pawn, pawn_color),
                    taken_piece: None,
                    promotion: None,
                };
                let moves = vec![m];
                Ok(builder.set_moves(moves))
            }
            Err(e) => Err(ForsythEdwardsNotationError::new(format!("unable to parse en passant square '{en_passent_option_string}' into a board position: {}", e)))
        }
    }
}

fn parse_half_turn_counter_from_string(
    builder: ChessGameBuilder,
    half_turn_counter_string: &str,
) -> Result<ChessGameBuilder, ForsythEdwardsNotationError> {
    match half_turn_counter_string.parse() {
        Ok(half_turn) => Ok(builder.set_fifty_move_rule_counter(half_turn)),
        Err(_) => Err(ForsythEdwardsNotationError::new(format!(
            "Unable to parse '{half_turn_counter_string}' into unsigned int for half turn count"
        ))),
    }
}

fn parse_turn_number_from_string(
    builder: ChessGameBuilder,
    turn_number_string: &str,
) -> Result<ChessGameBuilder, ForsythEdwardsNotationError> {
    match turn_number_string.parse() {
        Ok(turn_number) => Ok(builder.set_turn_number(turn_number)),
        Err(_) => Err(ForsythEdwardsNotationError::new(format!(
            "unable to parse '{turn_number_string}' into unsigned int for turn count"
        ))),
    }
}

fn get_board_as_fen_string(game: &ChessGame) -> String {
    let board = game.get_board();

    let board_as_fen_string: String = (0..board.get_height())
        .rev()
        .map(|rank| encode_row(board, rank))
        .collect::<Vec<String>>()
        .join("/");
    board_as_fen_string
}

fn encode_row(board: &Board<ChessPiece>, row: usize) -> String {
    let mut result = String::new();

    let mut empty_space_counter: usize = 0;

    for col in 0..board.get_width() {
        if let Some(piece) = board.get_piece_at_space(col, row) {
            if empty_space_counter != 0 {
                result.push_str(&empty_space_counter.to_string());
                empty_space_counter = 0;
            }
            result.push(piece.as_fen_char());
        } else {
            empty_space_counter += 1;
        }
        if empty_space_counter != 0 {
            result.push_str(&empty_space_counter.to_string());
        }
    }
    result
}

fn get_current_turn_char(game: &ChessGame) -> char {
    let current_turn = match game.get_current_players_turn() {
        White => 'w',
        Black => 'b',
    };
    current_turn
}

fn get_castling_rights(game: &ChessGame) -> String {
    let mut result = String::new();

    let (wq, wk, bq, bk) = game.get_castling_rights();

    if wq {
        result.push('Q');
    }
    if wk {
        result.push('K');
    }
    if bq {
        result.push('q');
    }
    if bk {
        result.push('k');
    }
    if result.is_empty() {
        result.push('-')
    };

    result
}

fn get_en_passent(game: &ChessGame) -> String {
    if let Some(EnPassant {
        new_position: (col, row),
        ..
    }) = game.get_last_move()
    {
        game_board::get_square_name_from_row_and_col(*col, *row)
    } else {
        String::from("-")
    }
}

pub struct ForsythEdwardsNotationError {
    reason: String,
}

impl ForsythEdwardsNotationError {
    fn new(reason: String) -> Self {
        Self { reason }
    }
}

impl Display for ForsythEdwardsNotationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Forsyth-Edwards Notation Error: {}", self.reason)
    }
}

impl Debug for ForsythEdwardsNotationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Forsyth-EdwardsNotationError: {}", self.reason)
    }
}

impl Error for ForsythEdwardsNotationError {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::piece::PieceType::{Bishop, King, Knight, Pawn, Queen, Rook};

    #[test]
    fn building_game_from_empty_string() {
        let res = build_game_from_string("");
        match res {
            Ok(_) => {
                panic!("expected error")
            }
            Err(e) => {
                assert_eq!(
                    "argument must be a string in Forsyth–Edwards Notation",
                    e.reason
                )
            }
        }
    }

    #[test]
    fn building_game_in_starting_position() {
        let starting_position_as_fen_string =
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        let game = build_game_from_string(starting_position_as_fen_string);
        assert_eq!(game.is_ok(), true);
        let game = game.unwrap();

        let expected_piece_type = [Rook, Knight, Bishop, Queen, King, Bishop, Knight, Rook];
        let board = game.get_board();
        for (col, expected_type) in expected_piece_type.iter().enumerate() {
            assert_eq!(
                board.get_piece_at_space(col, 0).unwrap(),
                &ChessPiece::new(*expected_type, White)
            );
            assert_eq!(
                board.get_piece_at_space(col, 1).unwrap(),
                &ChessPiece::new(Pawn, White)
            );
            assert!(board.get_piece_at_space(col, 2).is_none());
            assert!(board.get_piece_at_space(col, 3).is_none());
            assert!(board.get_piece_at_space(col, 4).is_none());
            assert!(board.get_piece_at_space(col, 5).is_none());
            assert_eq!(
                board.get_piece_at_space(col, 6).unwrap(),
                &ChessPiece::new(Pawn, Black)
            );
            assert_eq!(
                board.get_piece_at_space(col, 7).unwrap(),
                &ChessPiece::new(*expected_type, Black)
            );
        }

        assert_eq!(White, game.get_current_players_turn());
        assert_eq!((true, true, true, true), game.get_castling_rights());
        assert_eq!(0, game.get_moves().len());
        assert_eq!(0, game.get_50_move_rule_counter());
        assert_eq!(1, game.get_turn_number());
    }

    #[test]
    fn parse_fen_starting_position_to_board() {
        let mut game_builder = ChessGameBuilder::new();

        game_builder =
            parse_board_from_string(game_builder, "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR")
                .unwrap();
        game_builder = game_builder.set_current_turn(White);

        let expected_piece_type = [Rook, Knight, Bishop, Queen, King, Bishop, Knight, Rook];

        let game = game_builder.build().unwrap();
        let board = game.get_board();
        for (col, expected_type) in expected_piece_type.iter().enumerate() {
            assert_eq!(
                board.get_piece_at_space(col, 0).unwrap(),
                &ChessPiece::new(*expected_type, White)
            );
            assert_eq!(
                board.get_piece_at_space(col, 1).unwrap(),
                &ChessPiece::new(Pawn, White)
            );
            assert!(board.get_piece_at_space(col, 2).is_none());
            assert!(board.get_piece_at_space(col, 3).is_none());
            assert!(board.get_piece_at_space(col, 4).is_none());
            assert!(board.get_piece_at_space(col, 5).is_none());
            assert_eq!(
                board.get_piece_at_space(col, 6).unwrap(),
                &ChessPiece::new(Pawn, Black)
            );
            assert_eq!(
                board.get_piece_at_space(col, 7).unwrap(),
                &ChessPiece::new(*expected_type, Black)
            );
        }
    }

    #[test]
    fn parse_fen_board_from_invalid_string() {
        let game_builder = ChessGameBuilder::new();

        let result =
            parse_board_from_string(game_builder, "rnbqkbnr/ppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR");
        match result {
            Ok(_) => panic!("expected error"),
            Err(e) => {
                assert_eq!(
                    "File 'ppppppp' was not 8 spaces long in piece placement data",
                    e.reason
                )
            }
        }

        let starting_position_as_fen_string_missing_pawn =
            "rnbqkbnr/fppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR";
        let game_builder = ChessGameBuilder::new();

        let result =
            parse_board_from_string(game_builder, starting_position_as_fen_string_missing_pawn);
        match result {
            Ok(_) => panic!("expected error"),
            Err(e) => {
                assert_eq!(
                    "Unexpected char 'f' in file 'fppppppp' of piece placement data",
                    e.reason
                )
            }
        }
    }

    #[test]
    fn parse_fen_current_turn_string() {
        let mut game_builder = ChessGameBuilder::new();
        game_builder = parse_current_turn_from_string(game_builder, "w").unwrap();
        game_builder = game_builder.set_board(Board::build(1, 1).unwrap());

        let game = game_builder.build().unwrap();
        assert_eq!(White, game.get_current_players_turn());
    }

    #[test]
    fn parse_fen_invalid_current_turn_string() {
        let game_builder = ChessGameBuilder::new();
        match parse_current_turn_from_string(game_builder, "J") {
            Ok(_) => panic!("expected error"),
            Err(e) => assert_eq!("encountered unexpected token parsing turn from FEN string, Expected 'w' or 'b', received J", e.reason),
        }
    }

    #[test]
    fn parse_fen_no_castling_rights_string() {
        let mut game_builder = ChessGameBuilder::new();
        game_builder = game_builder.set_board(Board::build(1, 1).unwrap());
        game_builder = game_builder.set_current_turn(White);
        game_builder = parse_castling_rights_from_string(game_builder, "-").unwrap();

        let game = game_builder.build().unwrap();
        assert_eq!((false, false, false, false), game.get_castling_rights());
    }

    #[test]
    fn parse_fen_all_castling_rights_string() {
        let mut game_builder = ChessGameBuilder::new();
        game_builder = game_builder.set_board(Board::build(1, 1).unwrap());
        game_builder = game_builder.set_current_turn(White);
        game_builder = parse_castling_rights_from_string(game_builder, "KQkq").unwrap();

        let game = game_builder.build().unwrap();
        assert_eq!((true, true, true, true), game.get_castling_rights());
    }

    #[test]
    fn parse_fen_invalid_castling_rights_string() {
        let game_builder = ChessGameBuilder::new();
        match parse_castling_rights_from_string(game_builder, "KQn") {
            Ok(_) => panic!("expected error"),
            Err(e) => assert_eq!("Unexpected char 'n' in castling rights string", e.reason),
        }
    }

    #[test]
    fn parse_fen_en_passant_string() {
        // White Pawn
        let mut game_builder = ChessGameBuilder::new();
        game_builder = game_builder.set_board(Board::build(8, 8).unwrap());
        game_builder = game_builder.set_current_turn(White);
        game_builder = parse_en_passant_option_from_string(game_builder, "e3").unwrap();

        let game = game_builder.build().unwrap();
        if let Some(ChessMoveType::Move {
            original_position,
            new_position,
            piece,
            taken_piece,
            promotion,
        }) = game.get_last_move()
        {
            assert_eq!(&(4, 1), original_position);
            assert_eq!(&(4, 3), new_position);
            assert_eq!(ChessPiece::new(Pawn, White), *piece);
            assert!(taken_piece.is_none());
            assert!(promotion.is_none());
        }

        // Black Pawn
        let mut game_builder = ChessGameBuilder::new();
        game_builder = game_builder.set_board(Board::build(8, 8).unwrap());
        game_builder = game_builder.set_current_turn(White);
        game_builder = parse_en_passant_option_from_string(game_builder, "e6").unwrap();

        let game = game_builder.build().unwrap();
        if let Some(ChessMoveType::Move {
            original_position,
            new_position,
            piece,
            taken_piece,
            promotion,
        }) = game.get_last_move()
        {
            assert_eq!(&(4, 6), original_position);
            assert_eq!(&(4, 4), new_position);
            assert_eq!(ChessPiece::new(Pawn, Black), *piece);
            assert!(taken_piece.is_none());
            assert!(promotion.is_none());
        }
    }

    #[test]
    fn parse_fen_no_en_passant_string() {
        let mut game_builder = ChessGameBuilder::new();
        game_builder = game_builder.set_board(Board::build(8, 8).unwrap());
        game_builder = game_builder.set_current_turn(White);
        game_builder = parse_en_passant_option_from_string(game_builder, "-").unwrap();

        let game = game_builder.build().unwrap();
        assert!(game.get_last_move().is_none());
    }

    #[test]
    fn parse_fen_invalid_en_passant_string() {
        let game_builder = ChessGameBuilder::new();
        match parse_en_passant_option_from_string(game_builder, "_") {
            Ok(_) => panic!("expected error"),
            Err(e) => assert_eq!(
                "unable to parse en passant square '_' into a board position: Invalid input",
                e.reason
            ),
        }
    }

    #[test]
    fn parse_fen_half_move_counter_string() {
        let mut game_builder = ChessGameBuilder::new();
        game_builder = game_builder.set_board(Board::build(8, 8).unwrap());
        game_builder = game_builder.set_current_turn(White);
        game_builder = parse_half_turn_counter_from_string(game_builder, "32").unwrap();

        let game = game_builder.build().unwrap();
        assert_eq!(32, game.get_50_move_rule_counter());
    }

    #[test]
    fn parse_fen_invalid_move_counter_string() {
        let game_builder = ChessGameBuilder::new();
        match parse_half_turn_counter_from_string(game_builder, "_") {
            Ok(_) => panic!("expected error"),
            Err(e) => assert_eq!(
                "Unable to parse '_' into unsigned int for half turn count",
                e.reason
            ),
        }
    }

    #[test]
    fn parse_fen_turn_counter_string() {
        let mut game_builder = ChessGameBuilder::new();
        game_builder = game_builder.set_board(Board::build(8, 8).unwrap());
        game_builder = game_builder.set_current_turn(White);
        game_builder = parse_turn_number_from_string(game_builder, "15").unwrap();

        let game = game_builder.build().unwrap();
        assert_eq!(15, game.get_turn_number());
    }

    #[test]
    fn parse_fen_invalid_turn_counter_string() {
        let game_builder = ChessGameBuilder::new();
        match parse_turn_number_from_string(game_builder, "ns") {
            Ok(_) => panic!("expected error"),
            Err(e) => assert_eq!(
                "unable to parse 'ns' into unsigned int for turn count",
                e.reason
            ),
        }
    }
}
