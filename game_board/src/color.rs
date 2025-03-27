use std::fmt::Display;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum SquareColor {
    White,
    Black,
}

impl Display for SquareColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SquareColor::White => write!(f, "White"),
            SquareColor::Black => write!(f, "Black"),
        }
    }
}
