use crate::chess_game_move_analyzer;
use crate::piece::{ChessPiece, PieceType};
use crate::piece::PieceType::King;
use crate::ChessMoveType::Move;
use crate::{ChessGame, ChessMoveType, Color};
use game_board::Board;

/// Represents the current state of a chess game.
///
/// The `GameState` enum is used to track the status of an ongoing chess game.
/// It can be one of four possible states:
///
/// - `InProgress`: The game is actively being played, with available legal moves for the current turn.
/// - `Check`: The current player is in check, meaning their king is under threat but has legal moves to counter.
/// - `Checkmate`: The current player's king is in check and there are no legal moves to escape, resulting in a victory for the opponent.
/// - `Stalemate`: The game is in a state where the current player has no legal moves, but their king is not in check, resulting in a draw.
///
/// # Enum Variants
///
/// - `InProgress`: Holds a vector of legal moves and indicates whose turn it is.
/// - `Check`: Holds a vector of legal moves and indicates whose turn it is.
/// - `Checkmate`: Indicates the winning player's color.
/// - `Stalemate`: Indicates the game has ended in a draw.
#[derive(Debug, PartialEq)]
pub enum GameState {
    InProgress {
        legal_moves: Vec<ChessMoveType>,
        turn: Color,
    },
    Check {
        legal_moves: Vec<ChessMoveType>,
        turn: Color,
    },
    Checkmate {
        winner: Color,
    },
    Stalemate,
}


/// Determines the current state of a chess game.
///
/// The `get_game_state` function analyzes the chess game to determine
/// its current status. It will return one of the `GameState` variants:
///
/// - `InProgress`: If there are legal moves available and the game continues.
/// - `Check`: If the current player is in check but can still make legal moves.
/// - `Checkmate`: If the current player's king is in check and there are no legal
///   moves left, resulting in the opponent's victory.
/// - `Stalemate`: If the current player has no legal moves, and their king is not
///   in check, resulting in a draw.
///
/// # Parameters
///
/// - `game`: A mutable reference to the `ChessGame` struct representing the
///   game to be analyzed.
///
/// # Returns
///
/// - `GameState`: Enum variant representing the current state of the chess game.
pub fn get_game_state(game: &mut ChessGame) -> GameState {
    let legal_moves = chess_game_move_analyzer::get_legal_moves(game);
    if is_in_check(game.get_current_players_turn(), game.get_board()) {
        if legal_moves.is_empty() {
            GameState::Checkmate {
                winner: game.get_current_players_turn().opposite(),
            }
        } else {
            GameState::Check {
                legal_moves,
                turn: game.get_current_players_turn(),
            }
        }
    } else {
        if legal_moves.is_empty() {
            GameState::Stalemate
        } else {
            GameState::InProgress {
                legal_moves,
                turn: game.get_current_players_turn(),
            }
        }
    }
}


/// Checks if the player of the specified color is in check.
///
/// This function evaluates the board to determine if the player's king is under threat from any opposing pieces.
/// It scans through all pieces of the opposing color and checks if any have a legal move that can capture the king.
///
/// # Parameters
///
/// - `color`: The `Color` of the player whose king status is being checked.
/// - `board`: A reference to the `Board` containing chess pieces, representing the current state of the game.
///
/// # Returns
///
/// - `bool`: Returns `true` if the player's king is in check, meaning it is under threat. Returns `false` otherwise.
pub fn is_in_check(color: Color, board: &Board<ChessPiece>) -> bool {
    for row in 0..board.get_height() {
        for col in 0..board.get_width() {
            if let Some(piece) = board.get_piece_at_space(col, row) {
                if piece.get_color() == color.opposite() {
                    let moves = piece.possible_moves((col, row), board, None);
                    for m in moves {
                        match m {
                            Move { taken_piece, .. } => {
                                if let Some(taken_piece) = taken_piece {
                                    if taken_piece.get_piece_type() == King {
                                        return true;
                                    }
                                }
                            }
                            _ => return false,
                        }
                    }
                }
            }
        }
    }
    false
}


/// Determines if there is insufficient material on the board to continue the game.
///
/// The `is_insufficient_material` function checks if both players have insufficient material 
/// to reach a checkmate. The game would end in a draw if neither player can checkmate the opponent, 
/// regardless of the moves made.
///
/// # Parameters
///
/// - `board`: A reference to the `Board<ChessPiece>` containing chess pieces, representing 
///   the current state of the game.
///
/// # Returns
///
/// - `bool`: Returns `true` if both players have insufficient material to reach checkmate. 
///   Otherwise, it returns `false`.
pub fn is_insufficient_material(board: &Board<ChessPiece>) -> bool {
    let mut white_pieces = vec![];
    let mut black_pieces = vec![];
    for col in 0..board.get_width() {
        for row in 0..board.get_height() {
            if let Some(piece) = board.get_piece_at_space(col, row) {
                match piece.get_color() {
                    Color::White => white_pieces.push(piece),
                    Color::Black => black_pieces.push(piece),
                }
            }
        }
    }

    let check = |pieces: &Vec<&ChessPiece>| -> bool {
        if pieces.len() < 2 {
            return true;
        } else if pieces.len() == 2 {
            let piece_type_a = pieces[0].get_piece_type();
            let piece_type_b = pieces[1].get_piece_type();

            let other = if piece_type_a == King {piece_type_b} else {piece_type_a};
            return match other {
                PieceType::Knight => true,
                PieceType::Bishop => true,
                _ => false,
            }
        }
        return false
    };

    check(&white_pieces) && check(&black_pieces)
}

#[cfg(test)]
mod tests {
    use crate::chess_game_state_analyzer::GameState::{Check, Checkmate, InProgress, Stalemate};
    use crate::codec::forsyth_edwards_notation::build_game_from_string;
    use crate::Color::{Black, White};
    use super::*;

    #[test]
    fn game_with_starting_position_is_in_progress() {
        let mut game = ChessGame::new();
        match get_game_state(&mut game) {
            InProgress { legal_moves, turn } => {
                assert_eq!(legal_moves.len(), 20);
                assert_eq!(White, turn);
            },
            _ => panic!("Game state is not in progress."),
        };
    }

    #[test]
    fn game_in_check_has_legal_moves() {
        let mut game = build_game_from_string("4k3/8/8/8/8/8/8/r3R3 b - - 0 1").unwrap();
        match get_game_state(&mut game) {
            Check {legal_moves, turn} => {
                assert_eq!(turn, Black);
                assert_eq!(5, legal_moves.len())
            },
            _ => panic!("Game state is not in progress."),
        };
    }

    #[test]
    fn game_is_in_stalemate() {
        let mut game = build_game_from_string("k7/7R/8/8/8/8/8/1RK5 b - - 0 1").unwrap();
        match get_game_state(&mut game) {
            Stalemate {} => (),
            _ => panic!("Game state is not in progress."),
        }
    }

    #[test]
    fn game_is_in_check_mate() {
        let mut game = build_game_from_string("k6R/pp6/8/8/8/8/8/8 b - - 0 1").unwrap();
        match get_game_state(&mut game) {
            Checkmate {winner} => assert_eq!(White, winner),
            _ => panic!("Game state is not in progress."),
        }
    }

    #[test]
    fn game_with_starting_position_has_sufficient_material() {
        let game = ChessGame::new();
        let insufficient_material = is_insufficient_material(game.get_board());
        assert!(!insufficient_material);
    }

    #[test]
    fn one_player_has_insufficient_material_the_other_does_not() {
        let game = build_game_from_string("k7/8/8/8/8/8/8/KQ6 b - - 0 1").unwrap();
        let insufficient_material = is_insufficient_material(game.get_board());
        assert!(!insufficient_material);
    }

    #[test]
    fn only_kings_is_insufficient_material() {
        let game = build_game_from_string("k7/8/8/8/8/8/8/K7 b - - 0 1").unwrap();
        assert!(is_insufficient_material(game.get_board()));
    }

    #[test]
    fn king_and_bishop_or_knight_is_insufficient_material() {

        let game = build_game_from_string("k7/8/bN6/8/8/8/8/K7 b - - 0 1").unwrap();
        assert!(is_insufficient_material(game.get_board()));
    }
}
