use std::fmt;
use crate::pieces::peice::Piece;

pub struct BoardSquare {
	pub name: String,
	piece: Option<Box<dyn Piece>>,
	color: SquareColor,
}

pub enum SquareColor {
	White,
	Black,
}


#[derive(Eq, Hash, PartialEq, Debug)]
pub enum Direction {
	North,
	NorthEast,
	East,
	SouthEast,
	South,
	SouthWest,
	West,
	NorthWest,
}

impl BoardSquare {
	pub fn new(name: String, color: SquareColor) -> BoardSquare {
		BoardSquare {
			name,
			piece: None,
			color,
		}
	}

	pub fn set_piece(&mut self, piece: Box<dyn Piece>) {
		self.piece = Some(piece);
	}
}

impl fmt::Display for BoardSquare {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let bg_color = self.color.get_bg_color();
		match &self.piece {
			Some(piece) => write!(f, "{}{}{}", bg_color, piece, "\x1b[0m"),
			None => write!(f, "{}{}{}", bg_color, " ", "\x1b[0m"),
		}
	}
}

impl SquareColor {
	pub fn get_bg_color(&self) -> String {
		match self {
			SquareColor::White => String::from("\x1b[107m"),
			SquareColor::Black => String::from("\x1b[100m"),
		}
	}
}
