use crate::chess_game::ChessGame;
use crate::piece::ChessPiece;
use crate::Color;
use game_board::Board;

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
        '-', // TODO add En Passant string once i add move history to the game
        game.get_50_move_rule_counter(),
        game.get_turn_number()
    )
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
