use crate::ChessPiece;
use std::fmt::{Display, Formatter};

#[derive(Copy, Clone)]
pub struct ChessMove<'a> {
    pub original_position: (usize, usize),
    pub new_position: (usize, usize),
    pub piece: &'a ChessPiece,
    pub takes: Option<&'a ChessPiece>,
}

impl<'a> ChessMove<'a> {
    pub fn build(
        original_pos: (usize, usize),
        new_pos: (usize, usize),
        piece: &'a ChessPiece,
        takes: Option<&'a ChessPiece>,
    ) -> Self {
        ChessMove {
            original_position: original_pos,
            new_position: new_pos,
            piece,
            takes,
        }
    }
}

impl<'a> Display for ChessMove<'a> {
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
