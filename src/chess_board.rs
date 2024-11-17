use crate::chess_board_square::Square;
use crate::chess_piece::ChessPiece;
use crate::{Color, PieceType};
use std::fmt;

/// # Game Board struct
/// A struct used to keep track of the spaces of a rectangular game board made up of spaces
pub struct GameBoard {
    squares: Vec<Square>,
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

            if let Some(piece) = piece {
                let column = index % width;
                let row = index / width;

                let row = row as isize - (height - 1) as isize;
                let row = row.unsigned_abs();

                let index = column + row * width;
                board.squares[index].place_piece(piece);
            }
        }

        Ok(board)
    }

    pub fn chess_board_from_string(s: &str) -> Result<GameBoard, String> {
        Self::from_string(8, 8, s)
    }

    fn get_square_index(&self, col: usize, row: usize) -> usize {
        col + row * self.width
    }

    fn generate_board(width: usize, height: usize) -> Vec<Square> {
        assert!(width > 0 && height > 0);

        let mut spaces = vec![Square::build(0, 0); width * height];

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

    pub fn check_space(&self, col: usize, row: usize) -> Option<&ChessPiece> {
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
    pub fn place_piece(&mut self, piece: ChessPiece, col: usize, row: usize) {
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
    pub fn remove_piece(&mut self, col: usize, row: usize) -> Option<ChessPiece> {
        if col >= self.width {
            panic!("column outside of board bounds");
        }
        if row >= self.height {
            panic!("width is outside of board bounds");
        }
        let square_index = self.get_square_index(col, row);
        self.squares[square_index].clear_piece()
    }

    fn as_byte_arr(&self) -> Vec<u8> {
        let mut capacity = self.squares.len() / 2;
        if self.squares.len() % 2 == 1 {
            capacity += 1;
        }

        let mut byte_arr = vec![0b00000000; capacity];

        for col in 0..self.get_width() {
            for row in 0..self.get_height() {
                let square_index = self.get_square_index(col, row);
                let square = self.squares[square_index];

                let arr_index = square_index / 2;
                let first = square_index % 2 == 0;

                if first {
                    let byte = square.encode() << 4;
                    byte_arr[arr_index] |= byte;
                } else {
                    byte_arr[arr_index] |= square.encode();
                }
            }
        }

        byte_arr
    }
}

impl Clone for GameBoard {
    fn clone(&self) -> Self {
        Self {
            squares: self.squares.clone(),
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
                let square_index = self.get_square_index(x, y);
                let square = self.squares[square_index];
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
        for x in 0..10 {
            for y in 0..10 {
                let square_index = board.get_square_index(x, y);
                assert!(board.squares[square_index].get_piece().is_none());
            }
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

        let square_index = board.get_square_index(0, 0);
        assert!(board.squares[square_index].get_piece().is_some());
        assert_eq!(Knight, board.check_space(0, 0).unwrap().piece_type);
        assert_eq!(Color::White, board.check_space(0, 0).unwrap().color);

        let removed_piece = board.remove_piece(0, 0);
        assert!(removed_piece.is_some());
        assert!(board.squares[square_index].get_piece().is_none());

        board.place_piece(removed_piece.unwrap(), 0, 1);
        let square_index_a2 = board.get_square_index(0, 1);
        assert!(board.squares[square_index_a2].get_piece().is_some());
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
        board.remove_piece(8, 0).unwrap();
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
        println!("{board}");

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

    #[test]
    fn as_byte_arr_simple_board() {
        let chess_board_as_string = " ";
        let board = GameBoard::from_string(1, 1, chess_board_as_string).unwrap();

        let encoded = board.as_byte_arr();

        assert_eq!(1, encoded.len());
        assert_eq!(0b00000000, encoded[0]);
    }

    #[test]
    fn as_byte_arr_single_pawn() {
        let chess_board_as_string = "♙";
        let board = GameBoard::from_string(1, 1, chess_board_as_string).unwrap();

        let encoded = board.as_byte_arr();

        assert_eq!(1, encoded.len());
        assert_eq!(0b00100000, encoded[0]);
    }

    #[test]
    fn as_byte_arr_two_pieces() {
        let chess_board_as_string = "♙♟";
        let board = GameBoard::from_string(2, 1, chess_board_as_string).unwrap();

        let encoded = board.as_byte_arr();

        assert_eq!(1, encoded.len());
        assert_eq!(0b00100011, encoded[0]);
    }

    #[test]
    fn as_byte_arr_odd_number_of_squares() {
        let chess_board_as_string = "♛\n♚\n♜";
        let board = GameBoard::from_string(1, 3, chess_board_as_string).unwrap();

        let encoded = board.as_byte_arr();

        assert_eq!(2, encoded.len());
        assert_eq!(0b01011011, encoded[0]);
        assert_eq!(0b11010000, encoded[1]);
    }

    #[test]
    fn starting_position_as_byte_arr() {
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

        let encoded = board.as_byte_arr();

        assert_eq!(32, encoded.len());
        assert_eq!(0b01000110, encoded[0]);
        assert_eq!(0b10001100, encoded[1]);
        assert_eq!(0b10101000, encoded[2]);
        assert_eq!(0b01100100, encoded[3]);
        for white_pawns_index in 4..8 {
            assert_eq!(0b00100010, encoded[white_pawns_index]);
        }
        for empty_index in 9..24 {
            assert_eq!(0b00000000, encoded[empty_index]);
        }
        for black_pawn_index in 25..28 {
            assert_eq!(0b00110011, encoded[black_pawn_index]);
        }
        assert_eq!(0b01010111, encoded[28]);
        assert_eq!(0b10011101, encoded[29]);
        assert_eq!(0b10111001, encoded[30]);
        assert_eq!(0b01110101, encoded[31]);
    }
}
