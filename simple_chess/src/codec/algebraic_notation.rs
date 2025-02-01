use game_board::get_square_name_from_row_and_col;
use crate::ChessMoveType;
use crate::piece::{ChessPiece, PieceType};

pub fn encode_move_as_algebraic_notation(
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