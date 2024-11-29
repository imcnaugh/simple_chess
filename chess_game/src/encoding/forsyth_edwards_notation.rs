use crate::chess_game::ChessGame;
use crate::chess_game_builder::ChessGameBuilder;
use crate::piece::ChessPiece;
use crate::Color;
use game_board::Board;
use crate::ChessMoveType::EnPassant;

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

pub fn build_game_from_string(fen_string: &str) -> Result<ChessGame, &str> {
    let fen_string = fen_string.trim();
    if fen_string.is_empty() {
        return Err("argument must be a string in Forsyth–Edwards Notation");
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
            builder = step(builder, next);
        } else {
            return Err("Missing some parts of the string");
        }
    }

    builder.build()
}

fn parse_board_from_string(
    builder: ChessGameBuilder,
    board_as_fen_string: &str,
) -> ChessGameBuilder {
    todo!()
}

fn parse_current_turn_from_string(
    builder: ChessGameBuilder,
    current_turn_string: &str,
) -> ChessGameBuilder {
    todo!()
}

fn parse_castling_rights_from_string(
    builder: ChessGameBuilder,
    castling_rights_string: &str,
) -> ChessGameBuilder {
    todo!()
}

fn parse_en_passant_option_from_string(
    builder: ChessGameBuilder,
    en_passent_option_string: &str,
) -> ChessGameBuilder {
    todo!()
}

fn parse_half_turn_counter_from_string(
    builder: ChessGameBuilder,
    half_turn_counter_string: &str,
) -> ChessGameBuilder {
    todo!()
}

fn parse_turn_number_from_string(
    builder: ChessGameBuilder,
    turn_number_string: &str,
) -> ChessGameBuilder {
    todo!()
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
        Color::White => 'w',
        Color::Black => 'b',
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


/// Extracts the en passant target square for the last move in the chess game, if available.
///
/// # Returns
///
/// A `String` representing the algebraic notation of the target square for en passant capture,
/// such as 'e3'. If no en passant target square is available, returns a dash '-'.
///
/// # Arguments
///
/// * `game` - A reference to the `ChessGame` instance representing the current state of the game.
fn get_en_passent(game: &ChessGame) -> String {
    if let Some(EnPassant { new_position: (col, row), .. }) = game.get_last_move() {
        game_board::get_square_name_from_row_and_col(*col, *row)
    }  else {
        String::from("-")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn building_game_from_empty_string() {
        let res = build_game_from_string("");
        match res {
            Ok(_) => {
                panic!("expected error")
            }
            Err(e) => {
                assert_eq!("argument must be a string in Forsyth–Edwards Notation", e)
            }
        }
    }

    #[test]
    fn building_game_in_starting_position() {
        let starting_position_as_fen_string =
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        let game = build_game_from_string(starting_position_as_fen_string);
        assert_eq!(game.is_ok(), true);
        todo!("check the game is in the correct state")
    }
}
