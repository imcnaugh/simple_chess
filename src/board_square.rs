use std::collections::HashMap;
use std::fmt;
use std::rc::{Weak, Rc};
use std::cell::RefCell;

pub struct BoardSquare {
	pub name: String,
	piece: Option<String>,
	color: SquareColor,
	pub neighbors: HashMap<Direction, Weak<RefCell<BoardSquare>>>,
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
			neighbors: HashMap::new(),
		}
	}

	pub fn add_neighbor(&mut self, direction: Direction, square: &Rc<RefCell<BoardSquare>>){
		self.neighbors.insert(direction, Rc::downgrade(square));
	}
}

impl fmt::Display for BoardSquare {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let bg_color = self.color.get_bg_color();
        write!(f, "{}{}{}", bg_color, "   ", "\x1b[0m")
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
