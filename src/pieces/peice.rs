use std::fmt;

pub trait Piece: fmt::Display {
    fn color(&self) -> &PieceColor;
}

pub(crate) struct Pawn {
    color: PieceColor,
}
impl Pawn {
    pub fn new(color: PieceColor) -> Pawn {
        Pawn { color }
    }
}
impl Piece for Pawn {
    fn color(&self) -> &PieceColor {
        &self.color
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
impl Rook {
    pub fn new(color: PieceColor) -> Rook {
        Rook { color }
    }
}
impl Piece for Rook {
    fn color(&self) -> &PieceColor {
        &self.color
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
impl Knight {
    pub fn new(color: PieceColor) -> Knight {
        Knight { color }
    }
}
impl Piece for Knight {
    fn color(&self) -> &PieceColor {
        &self.color
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
impl Bishop {
    pub fn new(color: PieceColor) -> Bishop {
        Bishop { color }
    }
}
impl Piece for Bishop {
    fn color(&self) -> &PieceColor {
        &self.color
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
impl Queen {
    pub fn new(color: PieceColor) -> Queen {
        Queen { color }
    }
}
impl Piece for Queen {
    fn color(&self) -> &PieceColor {
        &self.color
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
impl King {
    pub fn new(color: PieceColor) -> King {
        King { color }
    }
}
impl Piece for King {
    fn color(&self) -> &PieceColor {
        &self.color
    }
}
impl fmt::Display for King {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let fg_color = self.color.get_fg_color();
        write!(f, "{}♚\x1b[0m", fg_color)
    }
}

pub(crate) enum PieceColor {
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
