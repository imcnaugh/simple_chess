use std::fmt;

pub enum ChessPiece {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn { _has_moved: bool },
}

pub enum PieceColor {
    White,
    Black,
}

pub struct Piece {
    pub(crate) piece: ChessPiece,
    pub(crate) color: PieceColor,
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let piece = match &self.piece {
            ChessPiece::King => "♔",
            ChessPiece::Queen => "♕",
            ChessPiece::Rook => "♖",
            ChessPiece::Bishop => "♗",
            ChessPiece::Knight => "♘",
            ChessPiece::Pawn { _has_moved } => "♙",
        };

        let color = match &self.color {
            PieceColor::White => "\x1b[97m",
            PieceColor::Black => "\x1b[30m",
        };

        write!(f, "{}{}", color, piece)
    }
}