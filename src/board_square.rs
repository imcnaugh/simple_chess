use std::fmt;

pub struct BoardSquare {
	name: String,
	piece: Option<String>,
	color: SquareColor,
}

impl BoardSquare {
	pub fn new(name: String, color: SquareColor) -> BoardSquare {
		BoardSquare {
			name,
			piece: None,
			color,
		}
	}
}

impl fmt::Display for BoardSquare {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let bg_color = self.color.get_bg_color();
        write!(f, "{}{}{}", bg_color, "   ", "\x1b[0m")
	}
}

pub enum SquareColor {
	White,
	Black,
}

impl SquareColor {
	pub fn get_bg_color(&self) -> String {
		match self {
			SquareColor::White => String::from("\x1b[107m"),
			SquareColor::Black => String::from("\x1b[100m"),
		}
	}
}
