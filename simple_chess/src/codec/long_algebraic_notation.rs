use game_board::get_square_name_from_row_and_col;
use crate::{ChessGame, ChessMoveType};
use crate::chess_game_state_analyzer::GameState;
use crate::piece::{ChessPiece, PieceType};

/// Encodes a chess move into long algebraic notation based on its type.
///
/// # Arguments
///
/// * `chess_move_type` - A reference to the `ChessMoveType` enum that represents
///   the type of move to be encoded.
///
/// # Returns
///
/// Returns a `String` containing the algebraic notation representation of the move.
///
/// # Examples
///
/// ```rust
/// use simple_chess::{ChessMoveType};
/// use simple_chess::piece::{PieceType, ChessPiece};
/// use simple_chess::codec::long_algebraic_notation::encode_move_as_long_algebraic_notation;
/// use simple_chess::Color::White;
/// let chess_move_type = ChessMoveType::Move {
///     original_position: (1, 1),
///     new_position: (1, 3),
///     piece: ChessPiece::new(PieceType::Pawn, White),
///     taken_piece: None,
///     promotion: None,
/// };
///
/// let notation = encode_move_as_long_algebraic_notation(&chess_move_type);
/// assert_eq!(notation, "b2b4");
/// ```
pub fn encode_move_as_long_algebraic_notation(
    chess_move_type: &ChessMoveType
) -> String {
    match chess_move_type {
        ChessMoveType::Move {
            original_position,
            new_position,
            piece,
            taken_piece,
            promotion
        } => encode_move(
            original_position,
            new_position,
            piece,
            taken_piece,
            promotion
        ),
        ChessMoveType::EnPassant {
            original_position,
            new_position,
            promotion,
            ..
        } => encode_en_passant(
            original_position,
            new_position,
            promotion,
        ),
        ChessMoveType::Castle {
            rook_original_position,
            ..
        } => encode_castle(
            rook_original_position
        ),
    }
}


/// Encodes the entire `ChessGame` instance into a single string using long algebraic notation.
///
/// This function takes the moves made in the game, encodes each move into long algebraic notation, 
/// and combines them into a single string separated by spaces.
///
/// # Arguments
///
/// * `game` - A reference to the `ChessGame` instance to be encoded.
///
/// # Returns
///
/// Returns a `String` containing the chess moves represented in long algebraic notation, 
/// separated by spaces.
///
/// # Examples
///
/// ```rust
/// use simple_chess::{ChessMoveType, ChessGame};
/// use simple_chess::piece::{PieceType, ChessPiece};
/// use simple_chess::codec::long_algebraic_notation::encode_game;
/// use simple_chess::Color::White;
///
/// let mut game = ChessGame::new();
/// let chess_move_type = ChessMoveType::Move {
///     original_position: (1, 1),
///     new_position: (1, 3),
///     piece: ChessPiece::new(PieceType::Pawn, White),
///     taken_piece: None,
///     promotion: None,
/// };
///
/// game.make_move(chess_move_type);
///
/// let notation = encode_game(&game);
/// assert_eq!(notation, "b2b4");
/// ```
pub fn encode_game(game: &ChessGame) -> String {
    game
        .get_moves()
        .iter()
        .map(|m| encode_move_as_long_algebraic_notation(&m))
        .collect::<Vec<String>>()
        .join(" ")
}


/// Builds a `ChessGame` instance from a string containing moves encoded in
/// long algebraic notation separated by spaces or newlines.
///
/// # Arguments
///
/// * `long_algebraic_notation_string` - A `&str` containing the moves in
///   long algebraic notation. Moves should be separated by spaces or newlines.
///
/// # Returns
///
/// Returns a `Result`:
/// - `Ok(ChessGame)` if the moves are valid and the game can be reconstructed.
/// - `Err(LongAlgebraicNotationError)` if an invalid move is encountered or the game cannot be built.
///
/// # Examples
///
/// ```rust
/// use simple_chess::chess_game_state_analyzer::GameState;
/// use simple_chess::codec::long_algebraic_notation::build_game_from_long_algebraic_notation;
///
/// let game_notation = "e2e4 e7e5\nNg1f3 Nb8c6";
/// let result = build_game_from_long_algebraic_notation(game_notation);
/// assert!(result.is_ok());
/// let mut game = result.unwrap_or_default();
/// match game.get_game_state() {
///     GameState::InProgress { .. } => assert!(true, "Game in Progress"),
///     _ => panic!("Game should be in progress"),
/// }
/// ```
pub fn build_game_from_long_algebraic_notation(long_algebraic_notation_string: &str) -> Result<ChessGame, LongAlgebraicNotationError> {
    let mut game = ChessGame::new();

    let normalized_string = long_algebraic_notation_string.trim().replace("\n", " ").to_lowercase();
    let moves = normalized_string.split(" ");
    
    for mv in moves {
        let available_moves = match game.get_game_state() {
            GameState::InProgress { legal_moves, .. } => legal_moves,
            GameState::Check { legal_moves, .. } => legal_moves,
            GameState::Checkmate { .. } => Vec::new(),
            GameState::Stalemate => Vec::new(),
        };

        let next_move = available_moves.iter().copied().find(|m| {
            encode_move_as_long_algebraic_notation(m).to_lowercase() == mv
        });

        match next_move {
            Some(m) => game.make_move(m),
            None => return Err(LongAlgebraicNotationError{reason: format!("Unable to make move {}", mv)})
        }
    }
    
    Ok(game)
}

fn encode_move(
    original_position: &(usize, usize),
    new_position: &(usize, usize),
    piece: &ChessPiece,
    taken_piece: &Option<ChessPiece>,
    promotion: &Option<ChessPiece>
) -> String {
    let moving_piece_str = get_piece_as_char(piece.get_piece_type()).unwrap_or(String::new());
    let original_square_str = get_square_name_from_row_and_col(original_position.0, original_position.1);
    let taken_str = match taken_piece {
        Some( .. ) => "x",
        None => ""
    };
    let new_square_str = get_square_name_from_row_and_col(new_position.0, new_position.1);
    let promotion_str = match promotion {
        Some(piece) => {
            format!("={}", get_piece_as_char(piece.get_piece_type()).unwrap())
        },
        None => String::new()
    };
    format!(
        "{}{}{}{}{}",
        moving_piece_str,
        original_square_str,
        taken_str,
        new_square_str,
        promotion_str
    )
}

fn encode_en_passant(
    original_position: &(usize, usize),
    new_position: &(usize, usize),
    promotion: &Option<ChessPiece>,
) -> String {
    let original_square_str = get_square_name_from_row_and_col(original_position.0, original_position.1);
    let new_square_str = get_square_name_from_row_and_col(new_position.0, new_position.1);
    let promotion_str = match promotion {
        None => String::new(),
        Some(piece) => format!("={}", get_piece_as_char(piece.get_piece_type()).unwrap())
    };
    format!("{}x{}{} e.p.",
        original_square_str,
        new_square_str,
        promotion_str,
    )
}

fn encode_castle(
    rook_original_position: &(usize, usize)
) -> String {
    if rook_original_position.0 == 0 {
        String::from("O-O-O")
    } else {
        String::from("O-O")
    }
}

fn get_piece_as_char(piece_type: PieceType) -> Option<String> {
    match piece_type {
        PieceType::Pawn => None,
        PieceType::Rook => Some(String::from("R")),
        PieceType::Knight => Some(String::from("N")),
        PieceType::Bishop => Some(String::from("B")),
        PieceType::Queen => Some(String::from("Q")),
        PieceType::King => Some(String::from("K")),
    }
}

pub struct LongAlgebraicNotationError {
    pub reason: String,
}

#[cfg(test)]
mod tests {
    use crate::codec::forsyth_edwards_notation::encode_game_as_string;
    use super::*;

    #[test]
    fn decode_string_to_game() {
        let string_to_decode = "h2h3 f7f6 Nb1a3 Ng8h6 g2g3 f6f5 c2c4 g7g6 g3g4 Nh6f7 g4xf5 ";

        match build_game_from_long_algebraic_notation(string_to_decode) {
            Ok(mut game) => {
                let current_state = &game.get_game_state();

                let game_as_fen_string = encode_game_as_string(&game);
                println!("{game_as_fen_string}");

                match current_state {
                    GameState::InProgress { .. } => assert!(true, "Game is in Progress"),
                    _ => panic!("Game should be in progress"),
                }
            }
            Err(_) => {
                panic!();
            }
        }
    }

    #[test]
    fn encode_game_to_string() {
        let mut game = ChessGame::new();
        let expected_encoded_string = String::from("Nb1c3 a7a6 Ra1b1 a6a5 Rb1a1 a5a4 Ra1b1 a4a3 Rb1a1 a3xb2");
        let mut expected_moves_as_encoded = expected_encoded_string.split(" ");

        for _ in 0..10 {
            let game_state = game.get_game_state();
            let next_move = match game_state {
                GameState::InProgress { legal_moves, .. } => legal_moves,
                GameState::Check { legal_moves, .. } => legal_moves,
                _ => panic!("Game has ended prematurely")
            }.first().unwrap().clone();

            let encoded_move = encode_move_as_long_algebraic_notation(&next_move);
            assert_eq!(expected_moves_as_encoded.next().unwrap(), encoded_move);

            game.make_move(next_move);
        }

        assert_eq!(expected_encoded_string, encode_game(&game))
    }
}