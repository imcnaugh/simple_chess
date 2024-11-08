use std::fmt::{Display, Formatter};
use crate::ChessPiece;

pub struct ChessMove<'a> {
    pub original_position: String,
    pub new_position: String,
    pub piece: &'a ChessPiece,
    pub takes: Option<&'a ChessPiece>,
}

impl<'a> ChessMove<'a> {
    pub fn build(original_pos: String, new_pos: String, piece: &'a ChessPiece, takes: Option<&'a ChessPiece>) -> Self {
        ChessMove{
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
            None => "Moves to".to_string()
        };
        write!(f, "{:?} {:?} {} {} from {}", self.piece.color, self.piece.piece_type, action, self.new_position, self.original_position)
    }
}