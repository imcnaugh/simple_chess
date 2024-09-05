use std::fmt;

pub trait Piece: fmt::Display {}

pub(crate) struct Pawn {
    color: PieceColor,
}
impl Piece for Pawn {}
impl Pawn {
    pub fn new(color: PieceColor) -> Pawn {
        Pawn { color }
    }
}
impl fmt::Display for Pawn {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let fg_color = self.color.get_fg_color();
        write!(f, "{}♟\x1b[0m", fg_color)
    }
}

pub(crate) struct Rook {
    color: PieceColor,
}
impl Piece for Rook {}
impl Rook {
    pub fn new(color: PieceColor) -> Rook {
        Rook { color }
    }
}
impl fmt::Display for Rook {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let fg_color = self.color.get_fg_color();
        write!(f, "{}♜\x1b[0m", fg_color)
    }
}

pub(crate) struct Knight {
    color: PieceColor,
}
impl Piece for Knight {}
impl Knight {
    pub fn new(color: PieceColor) -> Knight {
        Knight { color }
    }
}
impl fmt::Display for Knight {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let fg_color = self.color.get_fg_color();
        write!(f, "{}♞\x1b[0m", fg_color)
    }
}

pub(crate) struct Bishop {
    color: PieceColor,
}
impl Piece for Bishop {}
impl Bishop {
    pub fn new(color: PieceColor) -> Bishop {
        Bishop { color }
    }
}
impl fmt::Display for Bishop {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let fg_color = self.color.get_fg_color();
        write!(f, "{}♝\x1b[0m", fg_color)
    }
}

pub(crate) struct Queen {
    color: PieceColor,
}
impl Piece for Queen {}
impl Queen {
    pub fn new(color: PieceColor) -> Queen {
        Queen { color }
    }
}
impl fmt::Display for Queen {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let fg_color = self.color.get_fg_color();
        write!(f, "{}♛\x1b[0m", fg_color)
    }
}

pub(crate) struct King {
    color: PieceColor,
}
impl Piece for King {}
impl King {
    pub fn new(color: PieceColor) -> King {
        King { color }
    }
}
impl fmt::Display for King {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let fg_color = self.color.get_fg_color();
        write!(f, "{}♚\x1b[0m", fg_color)
    }
}

pub enum PieceColor {
    White,
    Black,
}

impl PieceColor {
    pub fn get_fg_color(&self) -> String {
        match self {
            PieceColor::White => String::from("\x1b[97m"),
            PieceColor::Black => String::from("\x1b[30m"),
        }
    }
}
