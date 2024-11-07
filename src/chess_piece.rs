use crate::Color;
use std::fmt;
use crate::chess_board::GameBoard;

/// # Enum for the type of chess piece.
#[derive(Debug, PartialEq)]
pub enum PieceType {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

/// # Struct for a chess piece.
pub struct ChessPiece {
    /// The color of the piece.
    pub color: Color,
    /// The type of piece.
    pub piece_type: PieceType,
}

impl ChessPiece {
    /// Creates a new chess piece.
    ///
    /// # Arguments
    /// * `color` - The color of the piece.
    /// * `piece_type` - The type of piece.
    ///
    /// # Returns
    /// A new chess piece.
    ///
    /// # Examples
    /// ```
    /// use chess::{ChessPiece, Color, PieceType};
    ///
    /// let white_pawn = ChessPiece::new(Color::White, PieceType::Pawn);
    /// ```
    pub fn new(color: Color, piece_type: PieceType) -> ChessPiece {
        ChessPiece { color, piece_type }
    }

    /// Gets the type of piece.
    ///
    /// # Returns
    /// The type of piece.
    ///
    /// # Examples
    /// ```
    /// use chess::{ChessPiece, Color, PieceType};
    ///
    /// let white_pawn = ChessPiece::new(Color::White, PieceType::Pawn);
    ///
    /// assert_eq!(white_pawn.get_piece_type(), &PieceType::Pawn);
    /// ```
    pub fn get_piece_type(&self) -> &PieceType {
        &self.piece_type
    }

    /// Gets the color of the piece.
    ///
    /// # Returns
    /// The color of the piece.
    ///
    /// # Examples
    /// ```
    /// use chess::{ChessPiece, Color, PieceType};
    ///
    /// let white_pawn = ChessPiece::new(Color::White, PieceType::Pawn);
    ///
    /// assert_eq!(white_pawn.get_color(), &Color::White);
    /// ```
    pub fn get_color(&self) -> &Color {
        &self.color
    }


    pub fn get_legal_moves(&self, col: usize, row: usize, board: &GameBoard) -> Vec<String> {
        let mut legal_moves = Vec::new();
        match self.piece_type {
            PieceType::Pawn => {}
            PieceType::Rook => {
                let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];
                for dir in directions.iter() {
                    let mut x = col as i32 + dir.0;
                    let mut y = row as i32 + dir.1;
                    while x >= 0 && y >= 0 && x < board.get_width() as i32 && y < board.get_height() as i32 {
                        if let Some(piece) = board.check_space(x as usize, y as usize) {
                            if *piece.get_color() != self.color {
                                legal_moves.push(format!("{}{}", (x as u8 + b'a') as char, (y as u8 + b'1') as char));
                            }
                            break;
                        }
                        legal_moves.push(format!("{}{}", (x as u8 + b'a') as char, (y as u8 + b'1') as char));
                        x += dir.0;
                        y += dir.1;
                    }
                }
            }
            PieceType::Knight => {}
            PieceType::Bishop => {}
            PieceType::Queen => {}
            PieceType::King => {}
        }
        legal_moves
    }
}

impl PieceType {
    fn get_as_utf_char(&self) -> char {
        match self {
            PieceType::Pawn => '♙',
            PieceType::Rook => '♖',
            PieceType::Knight => '♘',
            PieceType::Bishop => '♗',
            PieceType::Queen => '♕',
            PieceType::King => '♔',
        }
    }
}

impl fmt::Display for ChessPiece {
    /// Formats the chess piece as a UTF-8 character.
    ///
    /// # Returns
    /// The piece type as a string with a single UTF-8 character.
    ///
    /// # Examples
    /// ```
    /// use chess::{PieceType, ChessPiece, Color};
    ///
    /// let white_pawn = ChessPiece::new(Color::White, PieceType::Pawn);
    /// let black_pawn = ChessPiece::new(Color::Black, PieceType::Pawn);
    /// let white_rook = ChessPiece::new(Color::White, PieceType::Rook);
    /// let black_rook = ChessPiece::new(Color::Black, PieceType::Rook);
    /// let white_knight = ChessPiece::new(Color::White, PieceType::Knight);
    /// let black_knight = ChessPiece::new(Color::Black, PieceType::Knight);
    /// let white_bishop = ChessPiece::new(Color::White, PieceType::Bishop);
    /// let black_bishop = ChessPiece::new(Color::Black, PieceType::Bishop);
    /// let white_queen = ChessPiece::new(Color::White, PieceType::Queen);
    /// let black_queen = ChessPiece::new(Color::Black, PieceType::Queen);
    /// let white_king = ChessPiece::new(Color::White, PieceType::King);
    /// let black_king = ChessPiece::new(Color::Black, PieceType::King);
    ///
    /// assert_eq!(
    ///     "♙ ♖ ♘ ♗ ♕ ♔",
    ///     format!("{white_pawn} {white_rook} {white_knight} {white_bishop} {white_queen} {white_king}")
    /// );
    /// assert_eq!(
    ///     "♟ ♜ ♞ ♝ ♛ ♚",
    ///     format!("{black_pawn} {black_rook} {black_knight} {black_bishop} {black_queen} {black_king}")
    /// );
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut display_char = self.piece_type.get_as_utf_char();
        if self.color == Color::Black {
            display_char = char::from_u32(display_char as u32 + 6).unwrap();
        }

        write!(f, "{}", display_char,)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn white_pieces_display_correctly_in_ascii() {
        let white_pawn = ChessPiece::new(Color::White, PieceType::Pawn);
        let white_rook = ChessPiece::new(Color::White, PieceType::Rook);
        let white_knight = ChessPiece::new(Color::White, PieceType::Knight);
        let white_bishop = ChessPiece::new(Color::White, PieceType::Bishop);
        let white_queen = ChessPiece::new(Color::White, PieceType::Queen);
        let white_king = ChessPiece::new(Color::White, PieceType::King);

        assert_eq!(white_pawn.to_string(), "♙");
        assert_eq!(white_rook.to_string(), "♖");
        assert_eq!(white_knight.to_string(), "♘");
        assert_eq!(white_bishop.to_string(), "♗");
        assert_eq!(white_queen.to_string(), "♕");
        assert_eq!(white_king.to_string(), "♔");
    }

    #[test]
    fn black_pieces_display_correctly_in_ascii() {
        let black_pawn = ChessPiece::new(Color::Black, PieceType::Pawn);
        let black_rook = ChessPiece::new(Color::Black, PieceType::Rook);
        let black_knight = ChessPiece::new(Color::Black, PieceType::Knight);
        let black_bishop = ChessPiece::new(Color::Black, PieceType::Bishop);
        let black_queen = ChessPiece::new(Color::Black, PieceType::Queen);
        let black_king = ChessPiece::new(Color::Black, PieceType::King);

        assert_eq!(black_pawn.to_string(), "♟");
        assert_eq!(black_rook.to_string(), "♜");
        assert_eq!(black_knight.to_string(), "♞");
        assert_eq!(black_bishop.to_string(), "♝");
        assert_eq!(black_queen.to_string(), "♛");
        assert_eq!(black_king.to_string(), "♚");
    }

    #[test]
    fn rook_legal_moves() {
        let board_string = concat!("♜      \n", "       ");

        let board = GameBoard::from_string(7, 2, board_string).unwrap();

        let rook = board.check_space(0, 1).unwrap();

        let moves = rook.get_legal_moves(0, 1, &board);

        println!("{:?}", moves);
    }
}
