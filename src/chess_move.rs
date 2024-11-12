use crate::ChessPiece;
use std::fmt::{Display, Formatter};

#[derive(Copy, Clone)]
pub struct ChessMove {
    pub original_position: (usize, usize),
    pub new_position: (usize, usize),
    pub piece: ChessPiece,
    pub takes: Option<ChessPiece>,
    pub taken_piece_position: Option<(usize, usize)>
}

impl ChessMove {
    pub fn build(
        original_pos: (usize, usize),
        new_pos: (usize, usize),
        piece: ChessPiece,
        takes: Option<ChessPiece>,
        taken_piece_position: Option<(usize, usize)>,
    ) -> Self {
        ChessMove {
            original_position: original_pos,
            new_position: new_pos,
            piece,
            takes,
            taken_piece_position,
        }
    }
}

impl<'a> Display for ChessMove {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let action = match self.takes {
            Some(piece) => format!("Takes the {:?} {:?} at", piece.color, piece.piece_type),
            None => "Moves to".to_string(),
        };
        write!(
            f,
            "{:?} {:?} {} {} from {}",
            self.piece.color,
            self.piece.piece_type,
            action,
            format!(
                "{}{}",
                (self.new_position.0 as u8 + b'a') as char,
                (self.new_position.1 as u8 + b'1') as char
            ),
            format!(
                "{}{}",
                (self.original_position.0 as u8 + b'a') as char,
                (self.original_position.1 as u8 + b'1') as char
            )
        )
    }
}
