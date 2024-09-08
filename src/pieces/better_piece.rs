use std::fmt;

pub enum ChessPiece {
    King { _has_moved: bool },
    Queen,
    Rook { _has_moved: bool },
    Bishop,
    Knight,
    Pawn { _has_moved: bool },
}

#[derive(Clone)]
pub enum PieceColor {
    White,
    Black,
}

pub struct Piece {
    pub piece: ChessPiece,
    pub color: PieceColor,
    pub board_square_id: Option<String>,
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let piece = match &self.piece {
            ChessPiece::King { _has_moved } => "♔",
            ChessPiece::Queen => "♕",
            ChessPiece::Rook { _has_moved } => "♖",
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