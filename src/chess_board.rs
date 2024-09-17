use crate::chess_piece::ChessPiece;
use std::fmt;

/// # Game Board struct
/// A struct used to keep track of the spaces of a rectangular game board made up of spaces
pub struct GameBoard {
    squares: Vec<Option<ChessPiece>>,
    width: usize,
    height: usize,
}

impl GameBoard {
    /// Create a new board of any size,
    ///
    /// # Panics
    /// The function will panic if either height or width is 0
    pub fn build(width: usize, height: usize) -> GameBoard {
        GameBoard {
            squares: GameBoard::generate_board(width, height),
            width,
            height,
        }
    }

    /// Builds a chess board that is 8 x 8 spaces big
    pub fn build_chess_board() -> GameBoard {
        let board_width = 8;
        let board_height = 8;

        GameBoard {
            squares: GameBoard::generate_board(board_width, board_height),
            width: board_width,
            height: board_height,
        }
    }

    fn generate_board(width: usize, height: usize) -> Vec<Option<ChessPiece>> {
        assert!(width > 0 && height > 0);

        let mut spaces = Vec::new();

        for _ in 0..width * height {
            spaces.push(None);
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

    pub fn place_piece(&mut self, piece: ChessPiece, x: usize, y: usize) {
        if x >= self.width || y >= self.height {
            panic!("Out of bounds");
        }

        self.squares[x + y * self.width] = Some(piece);
    }

    pub fn remove_piece(&mut self, x: usize, y: usize) -> Option<ChessPiece> {
        if x >= self.width || y >= self.height {
            panic!("Out of bounds");
        }

        self.squares[x + y * self.width].take()
    }
}

impl fmt::Display for GameBoard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut board_string = String::new();

        let mut lines = vec![String::new(); self.get_height()];

        for y in 0..self.get_height() {
            for x in 0..self.get_width() {
                let index = (y * self.width) + x;
                let square_color = if (y + x) % 2 != 0 {
                    "\x1b[107m"
                } else {
                    "\x1b[100m"
                };

                let inner_char = match &self.squares[index] {
                    Some(piece) => format!("{}", piece),
                    None => " ".to_string(),
                };

                lines[y].push_str(format!("{} {} \x1b[0m", square_color, inner_char).as_str());
            }
        }

        for line in lines.iter().rev() {
            board_string.push_str(line.as_str());
            board_string.push('\n');
        }

        write!(f, "{}", board_string)
    }
}
