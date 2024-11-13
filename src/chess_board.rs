use crate::chess_piece::ChessPiece;
use crate::{Color, PieceType};
use std::fmt;

pub struct BoardSquare {
    column_index: usize,
    row_index: usize,
    piece: Option<ChessPiece>,
    color: Color,
}

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

        let mut board = GameBoard {
            squares: GameBoard::generate_board(board_width, board_height),
            width: board_width,
            height: board_height,
        };

        for i in 0..8 {
            board.place_piece(ChessPiece::new(Color::White, PieceType::Pawn), i, 1);
            board.place_piece(ChessPiece::new(Color::Black, PieceType::Pawn), i, 6);
        }

        board.place_piece(ChessPiece::new(Color::White, PieceType::Rook), 0, 0);
        board.place_piece(ChessPiece::new(Color::White, PieceType::Rook), 7, 0);
        board.place_piece(ChessPiece::new(Color::Black, PieceType::Rook), 0, 7);
        board.place_piece(ChessPiece::new(Color::Black, PieceType::Rook), 7, 7);
        board.place_piece(ChessPiece::new(Color::White, PieceType::Knight), 1, 0);
        board.place_piece(ChessPiece::new(Color::White, PieceType::Knight), 6, 0);
        board.place_piece(ChessPiece::new(Color::Black, PieceType::Knight), 1, 7);
        board.place_piece(ChessPiece::new(Color::Black, PieceType::Knight), 6, 7);
        board.place_piece(ChessPiece::new(Color::White, PieceType::Bishop), 2, 0);
        board.place_piece(ChessPiece::new(Color::White, PieceType::Bishop), 5, 0);
        board.place_piece(ChessPiece::new(Color::Black, PieceType::Bishop), 2, 7);
        board.place_piece(ChessPiece::new(Color::Black, PieceType::Bishop), 5, 7);
        board.place_piece(ChessPiece::new(Color::White, PieceType::Queen), 3, 0);
        board.place_piece(ChessPiece::new(Color::Black, PieceType::Queen), 3, 7);
        board.place_piece(ChessPiece::new(Color::White, PieceType::King), 4, 0);
        board.place_piece(ChessPiece::new(Color::Black, PieceType::King), 4, 7);

        board
    }

    pub fn from_string(width: usize, height: usize, s: &str) -> Result<GameBoard, String> {
        let s = s.replace('\n', "");

        if s.chars().count() != width * height {
            return Err(format!("expected a string of length: {} for a board with width: {}, and height: {}, received a string of {}", width * height, width, height, s.len()));
        }

        let mut board = GameBoard::build(width, height);

        for (index, char) in s.chars().enumerate() {
            let piece = match char {
                '♙' => Some(ChessPiece::new(Color::White, PieceType::Pawn)),
                '♖' => Some(ChessPiece::new(Color::White, PieceType::Rook)),
                '♘' => Some(ChessPiece::new(Color::White, PieceType::Knight)),
                '♗' => Some(ChessPiece::new(Color::White, PieceType::Bishop)),
                '♕' => Some(ChessPiece::new(Color::White, PieceType::Queen)),
                '♔' => Some(ChessPiece::new(Color::White, PieceType::King)),
                '♟' => Some(ChessPiece::new(Color::Black, PieceType::Pawn)),
                '♜' => Some(ChessPiece::new(Color::Black, PieceType::Rook)),
                '♞' => Some(ChessPiece::new(Color::Black, PieceType::Knight)),
                '♝' => Some(ChessPiece::new(Color::Black, PieceType::Bishop)),
                '♛' => Some(ChessPiece::new(Color::Black, PieceType::Queen)),
                '♚' => Some(ChessPiece::new(Color::Black, PieceType::King)),
                _ => None,
            };

            let column = index / width;
            let row = index % width;

            let column = column as isize - (height - 1) as isize;
            let column = column.abs();

            let index = (column as usize * width) + row;

            board.squares[index] = piece;
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

    pub fn check_space(&self, col: usize, row: usize) -> Option<&ChessPiece> {
        let index = col + (row * self.width);
        let board_square = &self.squares[index];
        match board_square {
            Some(piece) => Some(piece),
            None => None,
        }
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

impl Clone for GameBoard {
    fn clone(&self) -> Self {
        let squares = self.squares.to_vec();

        Self {
            squares,
            height: self.height,
            width: self.width,
        }
    }

    fn clone_from(&mut self, source: &Self) {
        self.squares = source.squares.clone();
        self.width = source.width;
        self.height = source.height;
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
                    // "\x1b[107m"
                    ""
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
    use crate::PieceType::{Bishop, King, Knight, Pawn, Queen, Rook};
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
        let piece = ChessPiece::new(Color::White, Knight);

        board.place_piece(piece, 0, 0);
        assert!(board.squares[0].is_some());
        assert_eq!(Knight, board.check_space(0, 0).unwrap().piece_type);
        assert_eq!(Color::White, board.check_space(0, 0).unwrap().color);

        let removed_piece = board.remove_piece(0, 0);
        assert!(removed_piece.is_some());
        assert!(board.squares[0].is_none());

        board.place_piece(removed_piece.unwrap(), 0, 1);
        assert!(board.squares[8].is_some());
        assert_eq!(Knight, board.check_space(0, 1).unwrap().piece_type);
        assert_eq!(Color::White, board.check_space(0, 1).unwrap().color);
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

    #[test]
    fn build_board_from_string() {
        let board_as_string = concat!(
            "        \n",
            "        \n",
            "        \n",
            "        \n",
            "        \n",
            "        \n",
            "        \n",
            "        \n"
        );

        let board = GameBoard::from_string(8, 8, board_as_string);

        assert!(board.is_ok());
        let game_board = board.unwrap();
        assert_eq!(game_board.get_width(), 8);
        assert_eq!(game_board.get_height(), 8);

        for x in 0..8 {
            for y in 0..8 {
                let piece = game_board.check_space(x, y);
                assert!(piece.is_none());
            }
        }
    }

    #[test]
    fn should_fail() {
        let board = GameBoard::from_string(8, 8, "");
        assert!(board.is_err());
    }

    #[test]
    fn should_be_able_to_detect_any_piece() {
        let board_string = concat!("♜♞♝♛♚♟ \n", "♖♘♗♕♔♙ ");

        let board = GameBoard::from_string(7, 2, board_string).unwrap();

        let pieces = [Rook, Knight, Bishop, Queen, King, Pawn];

        for col_index in 0..6 {
            let white_piece = board.check_space(col_index, 0);
            let black_piece = board.check_space(col_index, 1);

            assert_eq!(pieces[col_index], white_piece.unwrap().piece_type);
            assert_eq!(pieces[col_index], black_piece.unwrap().piece_type);

            assert_eq!(Color::White, white_piece.unwrap().color);
            assert_eq!(Color::Black, black_piece.unwrap().color);
        }

        assert!(board.check_space(6, 0).is_none());
        assert!(board.check_space(6, 1).is_none());
    }

    #[test]
    fn builds_starting_position_in_chess() {
        let board = GameBoard::build_chess_board();

        assert_eq!(8, board.height);
        assert_eq!(8, board.width);

        let pieces = [Rook, Knight, Bishop, Queen, King, Bishop, Knight, Rook];

        for col_index in 0..8 {
            assert_eq!(
                pieces[col_index],
                board.check_space(col_index, 0).unwrap().piece_type
            );
            assert_eq!(Color::White, board.check_space(col_index, 0).unwrap().color);
            assert_eq!(
                pieces[col_index],
                board.check_space(col_index, 7).unwrap().piece_type
            );
            assert_eq!(Color::Black, board.check_space(col_index, 7).unwrap().color);

            assert_eq!(Pawn, board.check_space(col_index, 1).unwrap().piece_type);
            assert_eq!(Color::White, board.check_space(col_index, 1).unwrap().color);
            assert_eq!(Pawn, board.check_space(col_index, 6).unwrap().piece_type);
            assert_eq!(Color::Black, board.check_space(col_index, 6).unwrap().color);

            for empty_row_index in 2..6 {
                assert!(board.check_space(col_index, empty_row_index).is_none());
            }
        }
    }

    #[test]
    fn build_starting_position_from_string() {
        let chess_board_as_string = concat!(
            "♜♞♝♛♚♝♞♜\n",
            "♟♟♟♟♟♟♟♟\n",
            "        \n",
            "        \n",
            "        \n",
            "        \n",
            "♙♙♙♙♙♙♙♙\n",
            "♖♘♗♕♔♗♘♖\n"
        );

        let board = GameBoard::from_string(8, 8, chess_board_as_string).unwrap();

        assert_eq!(8, board.height);
        assert_eq!(8, board.width);

        let pieces = [Rook, Knight, Bishop, Queen, King, Bishop, Knight, Rook];

        for col_index in 0..8 {
            assert_eq!(
                pieces[col_index],
                board.check_space(col_index, 0).unwrap().piece_type
            );
            assert_eq!(Color::White, board.check_space(col_index, 0).unwrap().color);
            assert_eq!(
                pieces[col_index],
                board.check_space(col_index, 7).unwrap().piece_type
            );
            assert_eq!(Color::Black, board.check_space(col_index, 7).unwrap().color);

            assert_eq!(Pawn, board.check_space(col_index, 1).unwrap().piece_type);
            assert_eq!(Color::White, board.check_space(col_index, 1).unwrap().color);
            assert_eq!(Pawn, board.check_space(col_index, 6).unwrap().piece_type);
            assert_eq!(Color::Black, board.check_space(col_index, 6).unwrap().color);

            for empty_row_index in 2..6 {
                assert!(board.check_space(col_index, empty_row_index).is_none());
            }
        }
    }
}
