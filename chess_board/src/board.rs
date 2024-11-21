use crate::square::Square;
use std::fmt;
use crate::piece::Piece;

/// # Game Board struct
/// A struct used to keep track of the spaces of a rectangular game board made up of spaces
pub struct Board {
    squares: Vec<Square>,
    width: usize,
    height: usize,
}

impl Board {
    /// Create a new board of any size,
    ///
    /// # Panics
    /// The function will panic if either height or width is 0
    pub fn build(width: usize, height: usize) -> Board {
        Board {
            squares: Board::generate_board(width, height),
            width,
            height,
        }
    }

    fn get_square_index(&self, col: usize, row: usize) -> usize {
        col + row * self.width
    }

    fn generate_board(width: usize, height: usize) -> Vec<Square> {
        assert!(width > 0 && height > 0);

        let mut spaces = Vec::with_capacity(width * height);

        for col in 0..width {
            for row in 0..height {
                let square = Square::build(col, row);
                let index = col + row * width;
                spaces[index] = square;
            }
        }

        spaces
    }

    /// the width of the board
    pub fn get_width(&self) -> usize {
        self.width
    }

    /// the height of the board
    pub fn get_height(&self) -> usize {
        self.height
    }

    pub fn check_space(&self, col: usize, row: usize) -> Option<&Box<dyn Piece>> {
        if col >= self.width {
            panic!("column outside of board bounds");
        }
        if row >= self.height {
            panic!("row is outside of board bounds");
        }
        let square_index = self.get_square_index(col, row);
        self.squares[square_index].get_piece()
    }

    /// Places a chess piece to the board
    pub fn place_piece(&mut self, piece: Box<dyn Piece>, col: usize, row: usize) {
        if col >= self.width {
            panic!("column outside of board bounds");
        }
        if row >= self.height {
            panic!("width is outside of board bounds");
        }
        let square_index = self.get_square_index(col, row);
        self.squares[square_index].place_piece(piece);
    }

    /// If the square identified has a chess piece, this removes it and returns ownership of that piece
    pub fn remove_piece(&mut self, col: usize, row: usize) -> Option<Box<dyn Piece>> {
        if col >= self.width {
            panic!("column outside of board bounds");
        }
        if row >= self.height {
            panic!("width is outside of board bounds");
        }
        let square_index = self.get_square_index(col, row);
        self.squares[square_index].clear_piece()
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut board_string = String::new();

        let mut lines = vec![String::new(); self.get_height()];

        for y in 0..self.get_height() {
            for x in 0..self.get_width() {
                let square_index = self.get_square_index(x, y);
                let square = &self.squares[square_index];
                lines[y].push_str(format!("{square}",).as_str());
            }
        }

        for line in lines.iter().rev() {
            board_string.push_str(line.as_str());
            board_string.push('\n');
        }

        write!(f, "{}", board_string)
    }
}
