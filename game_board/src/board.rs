use crate::piece::Piece;
use crate::square::Square;
use std::fmt;
use std::fmt::Display;

/// # Board
/// A struct used to keep track of a rectangular game board made up of spaces.
pub struct Board<P> {
    squares: Vec<Square<P>>,
    width: usize,
    height: usize,
}

impl<P> Board<P> {
    /// Create a new board of any size,
    ///
    /// # Panics
    /// The function will panic if either height or width is 0
    ///
    /// # Example
    /// ```
    ///use game_board::Board;
    ///
    ///let board = Board::build(8, 8);
    ///
    ///assert!(board.is_ok());
    /// ```
    pub fn build(width: usize, height: usize) -> Result<Board<P>, String> {
        Ok(Board {
            squares: Board::generate_board(width, height)?,
            width,
            height,
        })
    }

    /// the width of the board
    pub fn get_width(&self) -> usize {
        self.width
    }

    /// the height of the board
    pub fn get_height(&self) -> usize {
        self.height
    }
    
    /// get piece at square
    ///
    /// # Arguments
    ///
    /// * `col` - The column index (x-coordinate) of the space to check.
    /// * `row` - The row index (y-coordinate) of the space to check.
    ///
    /// # Returns
    ///
    /// * `Option<&Box<dyn Piece>>` - Some containing a reference to the piece if there
    ///   is one at the specified position, or None if the space is empty.
    ///
    /// # Panics
    ///
    /// This function will panic if the given column or row are outside the bounds
    /// of the board.
    ///
    /// # Example
    /// ```
    /// use std::any::Any;
    /// use game_board::{Board, Piece};
    ///
    /// enum Checker {
    ///     Red,
    ///     Black,
    /// }
    ///
    /// impl Piece for Checker {fn get_char_representation(&self) -> char {
    ///         'o'
    ///     }
    ///
    /// fn as_any(&self) -> &dyn Any {
    ///         self
    ///     }
    /// }
    ///
    /// let mut board = Board::build(10, 10).unwrap();
    ///
    /// let empty_space = board.get_piece_at_space(3,4);
    /// assert!(empty_space.is_none());
    ///
    /// board.place_piece(Box::new(Checker::Red) ,3, 4);
    ///
    /// let piece = board.get_piece_at_space(3, 4);
    /// assert!(piece.is_some())
    /// ```
    pub fn get_piece_at_space(&self, col: usize, row: usize) -> Option<&P> {
        self.validate_col_and_row(col, row);
        let square_index = self.get_square_index(col, row);
        self.squares[square_index].get_piece()
    }

    
    /// Places a piece at the given square
    ///
    /// # Arguments
    ///
    /// * `piece` - The piece to place on the board.
    /// * `col` - The column index (x-coordinate) where the piece will be placed.
    /// * `row` - The row index (y-coordinate) where the piece will be placed.
    ///
    /// # Panics
    ///
    /// This function will panic if the given column or row are outside the bounds
    /// of the board.
    ///
    /// # Example
    /// ```
    /// use game_board::{Board, Piece};
    ///
    /// enum Checker {
    ///     Red,
    ///     Black,
    /// }
    ///
    /// impl Piece for Checker {
    ///     fn get_char_representation(&self) -> char {
    ///         'o'
    ///     }
    ///
    ///     fn as_any(&self) -> &dyn std::any::Any {
    ///         self
    ///     }
    /// }
    ///
    /// let mut board = Board::build(10, 10).unwrap();
    ///
    /// board.place_piece(Box::new(Checker::Red), 3, 4);
    ///
    /// let piece = board.get_piece_at_space(3, 4);
    /// assert!(piece.is_some());
    /// ```
    pub fn place_piece(&mut self, piece: P, col: usize, row: usize) {
        self.validate_col_and_row(col, row);
        let square_index = self.get_square_index(col, row);
        self.squares[square_index].place_piece(piece);
    }


    /// Removes a piece from the given square
    ///
    /// # Arguments
    ///
    /// * `col` - The column index (x-coordinate) where the piece will be removed.
    /// * `row` - The row index (y-coordinate) where the piece will be removed.
    ///
    /// # Returns
    ///
    /// * `Option<Box<dyn Piece>>` - Some containing the removed piece if there was one,
    ///   or None if the space was already empty.
    ///
    /// # Panics
    ///
    /// This function will panic if the given column or row are outside the bounds
    /// of the board.
    ///
    /// # Example
    /// ```
    /// use game_board::{Board, Piece};
    ///
    /// enum Checker {
    ///     Red,
    ///     Black,
    /// }
    ///
    /// impl Piece for Checker {
    ///     fn get_char_representation(&self) -> char {
    ///         'o'
    ///     }
    ///
    ///     fn as_any(&self) -> &dyn std::any::Any {
    ///         self
    ///     }
    /// }
    ///
    /// let mut board = Board::build(10, 10).unwrap();
    ///
    /// board.place_piece(Box::new(Checker::Red), 3, 4);
    ///
    /// let piece = board.remove_piece(3, 4);
    /// assert!(piece.is_some());
    /// ```
    pub fn remove_piece(&mut self, col: usize, row: usize) -> Option<P> {
        self.validate_col_and_row(col, row);
        let square_index = self.get_square_index(col, row);
        self.squares[square_index].clear_piece()
    }

    fn generate_board(width: usize, height: usize) -> Result<Vec<Square<P>>, String> {
        if width == 0 || height == 0 {
            return Err(String::from(
                "Height and Width must be positive integers greater then 0",
            ));
        }

        let mut spaces = Vec::with_capacity(width * height);

        for row in 0..height {
            for col in 0..width {
                let square = Square::build(col, row);
                spaces.push(square);
            }
        }

        Ok(spaces)
    }

    fn get_square_index(&self, col: usize, row: usize) -> usize {
        col + row * self.width
    }

    fn validate_col_and_row(&self, col: usize, row: usize) {
        if col >= self.width {
            panic!("column outside of board bounds");
        }
        if row >= self.height {
            panic!("row is outside of board bounds");
        }
    }
}

impl<P: Display> Display for Board<P> {
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

#[cfg(test)]
mod tests {
    use std::any::Any;
    use super::*;
    
    struct MockPiece {}

    #[test]
    fn generate_8_by_8_board(){
        let board = Board::<MockPiece>::build(8,8);

        assert!(board.is_ok());

        let board = board.unwrap();

        assert_eq!(8, board.get_width());
        assert_eq!(8, board.get_height());

        assert_eq!(64, board.squares.len());

        // assert the squares on the board are empty
        for row in 0..board.get_height() {
            for col in 0..board.get_width() {
                assert!(board.get_piece_at_space(col, row).is_none());
            }
        }
    }

    #[test]
    fn can_not_make_board_with_width_of_0() {
        match Board::<MockPiece>::build(0, 8){
            Err(e) => assert_eq!("Height and Width must be positive integers greater then 0", e),
            _ => panic!("expected Err")
        };
    }

    #[test]
    fn can_not_make_board_with_height_or_width_of_0() {
        match Board::<MockPiece>::build(8, 0){
            Err(e) => assert_eq!("Height and Width must be positive integers greater then 0", e),
            _ => panic!("expected Err")
        };

        match Board::<MockPiece>::build(0, 8){
            Err(e) => assert_eq!("Height and Width must be positive integers greater then 0", e),
            _ => panic!("expected Err")
        };
    }

    #[test]
    fn can_place_retrieve_and_remove_piece() {
        struct ChessPawn {}
        impl Piece for ChessPawn {
            fn get_char_representation(&self) -> char {
                'p'
            }

            fn as_any(&self) -> &dyn Any {
                self
            }
        }

        let pawn = Box::new(ChessPawn {});
        let mut board = Board::build(8, 8).unwrap();
        assert!(board.get_piece_at_space(1,1).is_none());
        board.place_piece(pawn, 1, 1);
        let piece = board.get_piece_at_space(1, 1).unwrap();
        assert_eq!(piece.get_char_representation(), 'p');
        assert_eq!(board.remove_piece(1, 1).unwrap().get_char_representation(), 'p');
        assert!(board.get_piece_at_space(1,1).is_none());
    }

    #[test]
    #[should_panic]
    fn can_not_access_square_out_of_bounds_place_piece() {
        struct ChessPawn {}
        impl Piece for ChessPawn {
            fn get_char_representation(&self) -> char {
                'p'
            }

            fn as_any(&self) -> &dyn Any {
                self
            }
        }

        let pawn = Box::new(ChessPawn {});
        Board::build(1, 1).unwrap().place_piece(pawn, 0,1);
    }

    #[test]
    #[should_panic]
    fn can_not_access_square_out_of_bounds_get_piece() {
        Board::<MockPiece>::build(1, 1).unwrap().get_piece_at_space(0,1);
    }

    #[test]
    #[should_panic]
    fn can_not_access_square_out_of_bounds_remove_piece() {
        Board::<MockPiece>::build(1, 1).unwrap().remove_piece(0,1);
    }
}