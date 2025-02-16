use game_board::{get_square_name_from_row_and_col, Board};
use crate::{ChessGame, ChessGameBuilder, ChessMoveType};
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

pub fn build_game_from_long_algebraic_notation(long_algebraic_notation_string: &str) -> Result<ChessGame, String> {
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
            None => return Err(String::new())
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
        String::from("O-O")
    } else {
        String::from("O-O-O")
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

#[cfg(test)]
mod tests {
    use crate::chess_game::DrawReason;
    use crate::chess_game_state_analyzer::GameState;
    use crate::codec::forsyth_edwards_notation::encode_game_as_string;
    use super::*;

    #[test]
    fn decode_string_to_game() {
        let string_to_decode = "h2h3 f7f6
Nb1a3 Ng8h6
g2g3 f6f5
c2c4 g7g6
g3g4 Nh6f7
g4xf5 Bf8g7
Na3b1 h7h5
f5xg6 Ke8f8
Nb1c3 a7a5
Bf1g2 e7e6
Qd1a4 Bg7e5
Bg2c6 d7xc6
g6xf7 Qd8g5
b2b3 Be5g3
Qa4b4 Bg3d6
Qb4b5 Bd6c5
Nc3a4 Qg5g6
Qb5b4 Qg6d3
Bc1b2 Rh8h6
Bb2a3 a5xb4
Ke1f1 Nb8d7
Na4xc5 Qd3xe2
Kf1xe2 Ra8a7
Ra1d1 Ra7a5
Nc5d3 c6c5
Ke2e3 Ra5a8
Nd3e5 b4xa3
d2d3 Kf8e7
f7f8=N b7b6
Rd1a1 Ke7xf8
Ne5f7 Rh6h8
Ng1f3 Rh8g8
Ke3e2 Rg8g1
Nf7d6 Rg1xh1
Nd6b5 Nd7b8
Ke2d2 Rh1xa1
Kd2e3 Ra1e1
Ke3f4 Re1b1
Nf3g1 Nb8a6
Kf4e4 Rb1b2
h3h4 Rb2xa2
Ke4e3 Kf8g8
f2f3 Ra2h2
Ng1h3 Kg8f8
Nh3f4 a3a2
Nf4g2 Rh2h3
Nb5xc7 Rh3xf3
Ke3e2 Na6b4
Nc7d5 Ra8a7
Ke2xf3 e6e5
Ng2e1 Ra7a5
Nd5f4 a2a1=N
Nf4xh5 Ra5b5
Kf3e3 Bc8g4
Nh5g7 Bg4e2
Ng7f5 Nb4c6
b3b4 Be2d1
Nf5h6 Bd1c2
Nh6f5 Nc6xb4
Ke3e2 Bc2a4
Ke2f2 Nb4d5
h4h5 Rb5b1
Nf5e7 Nd5c7
h5h6 Kf8e8
Ne7d5 Ke8f7
Ne1c2 Ba4b5
c4xb5 Na1xc2
Kf2e2 Nc2b4
Nd5e3 Nc7e8
Ne3c4 Rb1d1
h6h7 Ne8f6
Nc4a5 Nb4d5
Ke2xd1 Kf7e8
Na5c6 Nd5c3
Kd1c1 Nf6g8
h7xg8=B Ke8f8
Nc6d4 Kf8e8
Kc1d2 c5xd4
Bg8d5 Nc3b1
Kd2d1 Ke8f8
Bd5b3 Kf8e7
Kd1e2 Ke7e8
Bb3g8 Ke8d7
Bg8e6 Kd7e7
Ke2f3 Ke7f8
Be6c8 Kf8f7
Bc8b7 Kf7e8
Bb7c6 Ke8e7
Bc6e4 Ke7f8
Kf3g3 Nb1c3
Kg3f3 Nc3d5
Be4f5 Kf8g8
Bf5e6 Kg8h8
Be6c8 Kh8g7
Bc8h3 Kg7h7
Kf3g2 Kh7g8
Bh3c8 e5e4
Bc8h3 Nd5e3
Kg2f2 Ne3f5
Bh3g2 Nf5e3
Bg2h1 Kg8g7
Bh1xe4 Ne3f5
Be4xf5 Kg7g8
Bf5g6 Kg8g7
Kf2g3 Kg7h6
Kg3g2 Kh6g5
Kg2f2 Kg5h6
Bg6e4 Kh6g7
Be4g2 Kg7h7
Kf2e1 Kh7g6
Bg2b7 Kg6f7
Bb7c8 Kf7f8
Ke1e2 Kf8e7
Ke2f3 Ke7d6
Kf3f2 Kd6c5
Kf2g2 Kc5xb5
Kg2h3 Kb5c6
Kh3g3 Kc6c5
Bc8e6 Kc5d6
Kg3g4 Kd6c7
Be6c8 Kc7xc8
Kg4f4 Kc8d7
Kf4e4 Kd7d8
Ke4f4 Kd8c8
Kf4g5 Kc8c7
Kg5f4 Kc7b7
Kf4e5 Kb7c8
Ke5d6 Kc8b7
Kd6e5 b6b5
Ke5f4 Kb7c7
Kf4g5 Kc7d7
Kg5f6 Kd7d8
Kf6g5 Kd8e7
Kg5h4 Ke7d8
Kh4g3 Kd8e8
Kg3g4 Ke8f8
Kg4g5 Kf8g8
Kg5f6 Kg8f8
Kf6e6 Kf8e8
Ke6d6 Ke8d8
Kd6c5 Kd8e7
Kc5b6 Ke7d7
Kb6b7 Kd7d8
Kb7a7 Kd8d7
Ka7a8 Kd7e8
Ka8b8 Ke8e7
Kb8a8 Ke7f8
Ka8b8 Kf8e7
Kb8c7 b5b4
Kc7c6 Ke7d8
Kc6d5 b4b3
Kd5e5 Kd8d7
Ke5e4 b3b2
Ke4e5 Kd7e7
Ke5f5 b2b1=Q
Kf5e4 Qb1g1
Ke4f4 Ke7d6
Kf4e4 Qg1g3
Ke4xd4 Qg3e3
Kd4c3 Qe3d2
Kc3xd2 Kd6d5
Kd2c3 Kd5c5
d3d4 Kc5d6
Kc3d2 Kd6d5
Kd2c1 Kd5e4
Kc1c2 Ke4f3
Kc2b3 Kf3e2
Kb3c3 Ke2f2
Kc3d2 Kf2g1
Kd2d3 Kg1g2
d4d5 Kg2f3
Kd3d4 Kf3g3
Kd4d3 Kg3f4
Kd3c3 Kf4e4
Kc3d2 Ke4f3
Kd2c1 Kf3f2
Kc1d2 Kf2g3
Kd2d3 Kg3g4
Kd3e3 Kg4f5
Ke3e2 Kf5e4
Ke2d1 Ke4d3
Kd1e1 Kd3c3
Ke1e2 Kc3b4
Ke2f2 Kb4c5
Kf2g1 Kc5d4
Kg1h1 Kd4d3
d5d6 Kd3d2
Kh1g1 Kd2e3
Kg1h2 Ke3f4
d6d7 Kf4e3
d7d8=R Ke3e2
Rd8d2 Ke2xd2";

        match build_game_from_long_algebraic_notation(string_to_decode) {
            Ok(mut game) => {
                let current_state = &game.get_game_state();

                let game_as_fen_string = encode_game_as_string(&game);
                println!("{game_as_fen_string}");

                match game.can_claim_draw() {
                    Some(draw_reason) => {
                        match draw_reason {
                            DrawReason::InsufficientPieces => assert!(true),
                            _ => assert!(false),
                        }
                    },
                    Nonek => assert!(false)
                }
            }
            Err(_) => {
                assert!(false);
            }
        }
    }
}