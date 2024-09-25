use crate::chess_piece::ChessPiece;
use crate::{Color, PieceType};
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

    pub fn from_string(width: usize, height: usize, s: &str) -> Result<GameBoard, String> {
        let s = s.replace('\n', "");

        if s.len() < width * height {
            return Err(format!("expected a string of at lest length: {} for a board with width: {}, and height: {}, received a string of {}", width * height, width, height, s.len()));
        }

        let mut board = GameBoard::build(width, height);

        for i in 0..(width * height) {
            let piece = match &s[i..i + 1] {
                "♙" => Some(ChessPiece::new(Color::White, PieceType::Pawn)),
                "♖" => Some(ChessPiece::new(Color::White, PieceType::Rook)),
                "♘" => Some(ChessPiece::new(Color::White, PieceType::Knight)),
                "♗" => Some(ChessPiece::new(Color::White, PieceType::Bishop)),
                "♕" => Some(ChessPiece::new(Color::White, PieceType::Queen)),
                "♔" => Some(ChessPiece::new(Color::White, PieceType::Knight)),
                "♟" => Some(ChessPiece::new(Color::Black, PieceType::Pawn)),
                "♜" => Some(ChessPiece::new(Color::Black, PieceType::Rook)),
                "♞" => Some(ChessPiece::new(Color::Black, PieceType::Knight)),
                "♝" => Some(ChessPiece::new(Color::Black, PieceType::Bishop)),
                "♛" => Some(ChessPiece::new(Color::Black, PieceType::Queen)),
                "♚" => Some(ChessPiece::new(Color::Black, PieceType::Knight)),
                _ => None,
            };
            board.squares[i] = piece;
        }

        Ok(board)
    }

    pub fn chess_board_from_string(s: &str) -> Result<GameBoard, String> {
        Self::from_string(8, 8, s)
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

    /// Places a chess piece to the board
    pub fn place_piece(&mut self, piece: ChessPiece, x: usize, y: usize) {
        if x >= self.width || y >= self.height {
            panic!("Out of bounds");
        }

        self.squares[x + y * self.width] = Some(piece);
    }

    /// If the square identified has a chess piece, this removes it and returns ownership of that piece
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Color, PieceType};

    #[test]
    fn test_build_game_board() {
        let board = GameBoard::build(10, 10);
        assert_eq!(board.get_width(), 10);
        assert_eq!(board.get_height(), 10);
        for i in 0..100 {
            assert!(board.squares[i].is_none());
        }
    }

    #[test]
    fn test_build_chess_board() {
        let board = GameBoard::build_chess_board();
        assert_eq!(board.get_width(), 8);
        assert_eq!(board.get_height(), 8);
    }

    #[test]
    fn test_place_and_remove_piece() {
        let mut board = GameBoard::build_chess_board();
        let piece = ChessPiece::new(Color::White, PieceType::Knight);

        board.place_piece(piece, 0, 0);
        assert!(board.squares[0].is_some());

        let removed_piece = board.remove_piece(0, 0);
        assert!(removed_piece.is_some());
        assert!(board.squares[0].is_none());

        board.place_piece(removed_piece.unwrap(), 0, 1);
        assert!(board.squares[8].is_some());
    }

    #[test]
    #[should_panic]
    fn test_place_piece_out_of_bounds() {
        let mut board = GameBoard::build_chess_board();
        let piece = ChessPiece::new(Color::White, PieceType::Knight);

        board.place_piece(piece, 8, 0);
    }

    #[test]
    #[should_panic]
    fn test_remove_piece_out_of_bounds() {
        let mut board = GameBoard::build_chess_board();

        board.remove_piece(8, 0);
    }
}
