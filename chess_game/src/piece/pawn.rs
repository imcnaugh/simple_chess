use crate::chess_move::ChessMoveType;
use crate::piece::{ChessPiece, PieceType};
use crate::Color;
use game_board::Board;

const PROMOTION_OPTIONS: [PieceType; 4] = [
    PieceType::Queen,
    PieceType::Rook,
    PieceType::Bishop,
    PieceType::Knight,
];

pub fn as_utf_str(color: Color) -> &'static str {
    match color {
        Color::White => "♙",
        Color::Black => "♟",
    }
}

pub fn as_fen_char(color: Color) -> char {
    match color {
        Color::White => 'P',
        Color::Black => 'p',
    }
}

pub fn possible_moves(
    color: Color,
    position: (usize, usize),
    board: &Board<ChessPiece>,
    last_move_type: Option<ChessMoveType>,
) -> Vec<ChessMoveType> {
    let mut possible_moves: Vec<ChessMoveType> = Vec::new();

    let forward_direction = match color {
        Color::White => 1,
        Color::Black => -1,
    };

    let promotion_row = match color {
        Color::White => board.get_height() - 1,
        Color::Black => 0,
    };

    let next_row = position.1 as i32 + forward_direction;
    if next_row < 0 || next_row > board.get_height() as i32 {
        return possible_moves;
    }

    // Simple move forward
    if board
        .get_piece_at_space(position.0, next_row as usize)
        .is_none()
    {
        possible_moves.append(&mut create_possible_moves(
            position,
            (position.0, next_row as usize),
            color,
            None,
            next_row as usize == promotion_row,
        ));

        // moving 2 spaces from starting row
        let starting_row = match color {
            Color::White => 1,
            Color::Black => board.get_height() - 2,
        };
        if position.1 == starting_row {
            let double_next_row = position.1 as i32 + 2 * forward_direction;
            if double_next_row >= 0 && double_next_row < board.get_height() as i32 {
                if board
                    .get_piece_at_space(position.0, double_next_row as usize)
                    .is_none()
                {
                    possible_moves.append(&mut create_possible_moves(
                        position,
                        (position.0, double_next_row as usize),
                        color,
                        None,
                        next_row as usize == promotion_row,
                    ));
                }
            }
        }
    }

    // Taking to the left
    if position.0 > 0 {
        if let Some(piece) = board.get_piece_at_space(position.0 - 1, next_row as usize) {
            possible_moves.append(&mut create_possible_moves(
                position,
                (position.0 - 1, next_row as usize),
                color,
                Some(piece.clone()),
                next_row as usize == promotion_row,
            ));
        }

        // En Passant
        if let Some(ChessMoveType::Move {
            piece,
            new_position,
            original_position,
            ..
        }) = last_move_type
        {
            if piece.piece_type == PieceType::Pawn {
                let rows_moved = if original_position.1 < new_position.1 {
                    new_position.1 - original_position.1
                } else {
                    original_position.1 - new_position.1
                };

                if rows_moved == 2
                    && new_position.0 == position.0 - 1
                    && new_position.1 == position.1
                {
                    possible_moves.append(&mut create_possible_en_passant(
                        position,
                        (position.0 - 1, next_row as usize),
                        color,
                        piece,
                        new_position,
                        next_row as usize == promotion_row,
                    ));
                }
            }
        }
    }

    // Taking to the right
    if position.0 < board.get_width() - 1 {
        if let Some(piece) = board.get_piece_at_space(position.0 + 1, next_row as usize) {
            possible_moves.append(&mut create_possible_moves(
                position,
                (position.0 + 1, next_row as usize),
                color,
                Some(piece.clone()),
                next_row as usize == promotion_row,
            ));
        }

        // En Passant
        if let Some(ChessMoveType::Move {
            piece,
            new_position,
            original_position,
            ..
        }) = last_move_type
        {
            if piece.piece_type == PieceType::Pawn {
                let rows_moved = if original_position.1 < new_position.1 {
                    new_position.1 - original_position.1
                } else {
                    original_position.1 - new_position.1
                };

                if rows_moved == 2
                    && new_position.0 == position.0 + 1
                    && new_position.1 == position.1
                {
                    possible_moves.append(&mut create_possible_en_passant(
                        position,
                        (position.0 + 1, next_row as usize),
                        color,
                        piece,
                        new_position,
                        next_row as usize == promotion_row,
                    ));
                }
            }
        }
    }

    possible_moves
}

fn create_possible_moves(
    original_position: (usize, usize),
    new_position: (usize, usize),
    color: Color,
    taken_piece: Option<ChessPiece>,
    can_promote: bool,
) -> Vec<ChessMoveType> {
    let mut possible_moves: Vec<ChessMoveType> = Vec::new();
    if can_promote {
        for option in PROMOTION_OPTIONS.iter() {
            possible_moves.push(ChessMoveType::Move {
                original_position,
                new_position,
                piece: ChessPiece::new(PieceType::Pawn, color),
                taken_piece,
                promotion: Some(ChessPiece::new(*option, color)),
            })
        }
    } else {
        possible_moves.push(ChessMoveType::Move {
            original_position,
            new_position,
            piece: ChessPiece::new(PieceType::Pawn, color),
            taken_piece,
            promotion: None,
        })
    }

    possible_moves
}

fn create_possible_en_passant(
    original_position: (usize, usize),
    new_position: (usize, usize),
    color: Color,
    taken_piece: ChessPiece,
    taken_piece_position: (usize, usize),
    can_promote: bool,
) -> Vec<ChessMoveType> {
    let mut possible_en_passants: Vec<ChessMoveType> = Vec::new();

    if can_promote {
        for option in PROMOTION_OPTIONS.iter() {
            possible_en_passants.push(ChessMoveType::EnPassant {
                original_position,
                new_position,
                piece: ChessPiece::new(PieceType::Pawn, color),
                taken_piece,
                taken_piece_position,
                promotion: Some(ChessPiece::new(*option, color)),
            })
        }
    } else {
        possible_en_passants.push(ChessMoveType::EnPassant {
            original_position,
            new_position,
            piece: ChessPiece::new(PieceType::Pawn, color),
            taken_piece,
            taken_piece_position,
            promotion: None,
        })
    }

    possible_en_passants
}
