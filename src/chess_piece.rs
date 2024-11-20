use crate::chess_board::GameBoard;
use crate::chess_board_square::SquareId;
use crate::chess_move::ChessMoveType;
use crate::chess_move::ChessMoveType::{EnPassant, Move};
use crate::Color;
use crate::PieceType::{Bishop, King, Knight, Pawn, Queen, Rook};
use std::fmt;

/// # Enum for the type of chess piece.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum PieceType {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

/// # Struct for a chess piece.
#[derive(Copy, Clone)]
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

    pub fn get_legal_moves<'a>(
        &'a self,
        col: usize,
        row: usize,
        board: &'a GameBoard,
        last_move: Option<&ChessMoveType>,
    ) -> Vec<ChessMoveType> {
        let mut legal_moves: Vec<ChessMoveType> = Vec::new();
        match self.piece_type {
            Pawn => {
                let promotion_row = match self.color {
                    Color::White => board.get_height() - 1,
                    Color::Black => 0,
                };

                let one_ahead = match self.color {
                    Color::White => row + 1,
                    Color::Black => row - 1,
                };

                if board.check_space(col, one_ahead).is_none() {
                    legal_moves.append(&mut create_moves(
                        (col, row),
                        (col, one_ahead),
                        *self,
                        None,
                        one_ahead == promotion_row,
                    ));

                    let starting_row = match self.color {
                        Color::White => 1,
                        Color::Black => board.get_height() - 2,
                    };

                    if row == starting_row {
                        let two_ahead = match self.color {
                            Color::White => row + 2,
                            Color::Black => row - 2,
                        };

                        if board.check_space(col, two_ahead).is_none() {
                            legal_moves.append(&mut create_moves(
                                (col, row),
                                (col, two_ahead),
                                *self,
                                None,
                                one_ahead == promotion_row,
                            ));
                        }
                    }
                }

                if col > 0 {
                    if let Some(piece) = board.check_space(col - 1, one_ahead) {
                        if *piece.get_color() != self.color {
                            legal_moves.append(&mut create_moves(
                                (col, row),
                                (col - 1, one_ahead),
                                *self,
                                Some(*piece),
                                one_ahead == promotion_row,
                            ));
                        }
                    }

                    // En Passant
                    if let Some(Move {
                        original_position,
                        new_position,
                        piece,
                        ..
                    }) = last_move
                    {
                        if piece.piece_type == PieceType::Pawn {
                            let rows_moved = if original_position.get_row() < new_position.get_row()
                            {
                                new_position.get_row() - original_position.get_row()
                            } else {
                                original_position.get_row() - new_position.get_row()
                            };

                            if rows_moved == 2
                                && new_position.get_row() == row
                                && new_position.get_column() == col - 1
                            {
                                legal_moves.push(EnPassant {
                                    original_position: SquareId::build(col, row),
                                    new_position: SquareId::build(col - 1, one_ahead),
                                    piece: *self,
                                    taken_piece: *piece,
                                    taken_piece_position: SquareId::build(col - 1, row),
                                })
                            }
                        }
                    }
                }

                if col < board.get_width() - 1 {
                    if let Some(piece) = board.check_space(col + 1, one_ahead) {
                        if *piece.get_color() != self.color {
                            legal_moves.append(&mut create_moves(
                                (col, row),
                                (col + 1, one_ahead),
                                *self,
                                Some(*piece),
                                one_ahead == promotion_row,
                            ));
                        }
                    }

                    // En Passant
                    if let Some(Move {
                        original_position,
                        new_position,
                        piece,
                        ..
                    }) = last_move
                    {
                        if piece.piece_type == PieceType::Pawn {
                            let rows_moved = if original_position.get_row() < new_position.get_row()
                            {
                                new_position.get_row() - original_position.get_row()
                            } else {
                                original_position.get_row() - new_position.get_row()
                            };

                            if rows_moved == 2
                                && new_position.get_row() == row
                                && new_position.get_column() == col + 1
                            {
                                legal_moves.push(EnPassant {
                                    original_position: SquareId::build(col, row),
                                    new_position: SquareId::build(col + 1, one_ahead),
                                    piece: *self,
                                    taken_piece: *piece,
                                    taken_piece_position: SquareId::build(col + 1, row),
                                })
                            }
                        }
                    }
                }
            }
            Rook => {
                let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];
                for dir in directions.iter() {
                    let mut x = col as i32 + dir.0;
                    let mut y = row as i32 + dir.1;
                    while x >= 0
                        && y >= 0
                        && x < board.get_width() as i32
                        && y < board.get_height() as i32
                    {
                        if let Some(piece) = board.check_space(x as usize, y as usize) {
                            if *piece.get_color() != self.color {
                                legal_moves.push(Move {
                                    original_position: SquareId::build(col, row),
                                    new_position: SquareId::build(x as usize, y as usize),
                                    piece: *self,
                                    taken_piece: Some(*piece),
                                    promotion: None,
                                })
                            }
                            break;
                        }
                        legal_moves.push(Move {
                            original_position: SquareId::build(col, row),
                            new_position: SquareId::build(x as usize, y as usize),
                            piece: *self,
                            taken_piece: None,
                            promotion: None,
                        });

                        x += dir.0;
                        y += dir.1;
                    }
                }
            }
            Knight => {
                let moves = [
                    (1, 2),
                    (1, -2),
                    (-1, 2),
                    (-1, -2),
                    (2, 1),
                    (2, -1),
                    (-2, 1),
                    (-2, -1),
                ];
                for mv in moves.iter() {
                    let x = col as i32 + mv.0;
                    let y = row as i32 + mv.1;
                    if x >= 0
                        && y >= 0
                        && x < board.get_width() as i32
                        && y < board.get_height() as i32
                    {
                        if let Some(piece) = board.check_space(x as usize, y as usize) {
                            if *piece.get_color() != self.color {
                                legal_moves.push(Move {
                                    original_position: SquareId::build(col, row),
                                    new_position: SquareId::build(x as usize, y as usize),
                                    piece: *self,
                                    taken_piece: Some(*piece),
                                    promotion: None,
                                });
                            }
                        } else {
                            legal_moves.push(Move {
                                original_position: SquareId::build(col, row),
                                new_position: SquareId::build(x as usize, y as usize),
                                piece: *self,
                                taken_piece: None,
                                promotion: None,
                            });
                        }
                    }
                }
            }
            Bishop => {
                let directions = [(1, 1), (1, -1), (-1, 1), (-1, -1)];
                for dir in directions.iter() {
                    let mut x = col as i32 + dir.0;
                    let mut y = row as i32 + dir.1;
                    while x >= 0
                        && y >= 0
                        && x < board.get_width() as i32
                        && y < board.get_height() as i32
                    {
                        if let Some(piece) = board.check_space(x as usize, y as usize) {
                            if *piece.get_color() != self.color {
                                legal_moves.push(Move {
                                    original_position: SquareId::build(col, row),
                                    new_position: SquareId::build(x as usize, y as usize),
                                    piece: *self,
                                    taken_piece: Some(*piece),
                                    promotion: None,
                                });
                            }
                            break;
                        }
                        legal_moves.push(Move {
                            original_position: SquareId::build(col, row),
                            new_position: SquareId::build(x as usize, y as usize),
                            piece: *self,
                            taken_piece: None,
                            promotion: None,
                        });
                        x += dir.0;
                        y += dir.1;
                    }
                }
            }
            Queen => {
                let directions = [
                    (0, 1),
                    (0, -1),
                    (1, 0),
                    (-1, 0),
                    (1, 1),
                    (1, -1),
                    (-1, 1),
                    (-1, -1),
                ];
                for dir in directions.iter() {
                    let mut x = col as i32 + dir.0;
                    let mut y = row as i32 + dir.1;
                    while x >= 0
                        && y >= 0
                        && x < board.get_width() as i32
                        && y < board.get_height() as i32
                    {
                        if let Some(piece) = board.check_space(x as usize, y as usize) {
                            if *piece.get_color() != self.color {
                                legal_moves.push(Move {
                                    original_position: SquareId::build(col, row),
                                    new_position: SquareId::build(x as usize, y as usize),
                                    piece: *self,
                                    taken_piece: Some(*piece),
                                    promotion: None,
                                });
                            }
                            break;
                        }
                        legal_moves.push(Move {
                            original_position: SquareId::build(col, row),
                            new_position: SquareId::build(x as usize, y as usize),
                            piece: *self,
                            taken_piece: None,
                            promotion: None,
                        });
                        x += dir.0;
                        y += dir.1;
                    }
                }
            }
            King => {
                let moves = [
                    (0, 1),
                    (0, -1),
                    (1, 0),
                    (-1, 0),
                    (1, 1),
                    (1, -1),
                    (-1, 1),
                    (-1, -1),
                ];
                for mv in moves.iter() {
                    let x = col as i32 + mv.0;
                    let y = row as i32 + mv.1;
                    if x >= 0
                        && y >= 0
                        && x < board.get_width() as i32
                        && y < board.get_height() as i32
                    {
                        if let Some(piece) = board.check_space(x as usize, y as usize) {
                            if *piece.get_color() != self.color {
                                legal_moves.push(Move {
                                    original_position: SquareId::build(col, row),
                                    new_position: SquareId::build(x as usize, y as usize),
                                    piece: *self,
                                    taken_piece: Some(*piece),
                                    promotion: None,
                                });
                            }
                        } else {
                            legal_moves.push(Move {
                                original_position: SquareId::build(col, row),
                                new_position: SquareId::build(x as usize, y as usize),
                                piece: *self,
                                taken_piece: None,
                                promotion: None,
                            });
                        }
                    }
                }
            }
        }
        legal_moves
    }

    pub fn get_notation_char(&self) -> &str {
        match self.piece_type {
            Pawn => "",
            Rook => "R",
            Knight => "N",
            Bishop => "B",
            Queen => "Q",
            King => "K",
        }
    }

    pub fn get_as_4_bit(&self) -> u8 {
        let mut bits = match self.piece_type {
            Pawn => 0b0010,
            Rook => 0b0100,
            Knight => 0b0110,
            Bishop => 0b1000,
            King => 0b1010,
            Queen => 0b1100,
        };
        bits |= match self.color {
            Color::White => 0b0000,
            Color::Black => 0b0001,
        };
        bits
    }
}

fn create_moves(
    o_pos: (usize, usize),
    n_pos: (usize, usize),
    piece: ChessPiece,
    taken_piece: Option<ChessPiece>,
    can_promote: bool,
) -> Vec<ChessMoveType> {
    let mut moves = Vec::new();
    if can_promote {
        let promotion_options = [Queen, Rook, Knight, Bishop];
        for promotion_piece in promotion_options {
            moves.push(Move {
                original_position: SquareId::build(o_pos.0, o_pos.1),
                new_position: SquareId::build(n_pos.0, n_pos.1),
                piece,
                taken_piece,
                promotion: Some(ChessPiece::new(piece.color, promotion_piece)),
            })
        }
    } else {
        moves.push(Move {
            original_position: SquareId::build(o_pos.0, o_pos.1),
            new_position: SquareId::build(n_pos.0, n_pos.1),
            piece,
            taken_piece,
            promotion: None,
        })
    }
    moves
}

impl PieceType {
    fn get_as_utf_char(&self) -> char {
        match self {
            Pawn => '♙',
            Rook => '♖',
            Knight => '♘',
            Bishop => '♗',
            Queen => '♕',
            King => '♔',
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

        let moves = rook.get_legal_moves(0, 1, &board, None);

        for m in moves {
            println!("{}", m);
        }
    }

    #[test]
    fn rook_legal_moves_with_taking() {
        let board_string = concat!("♜  ♟   \n", "♘      ");

        let board = GameBoard::from_string(7, 2, board_string).unwrap();

        let rook = board.check_space(0, 1).unwrap();

        let moves = rook.get_legal_moves(0, 1, &board, None);

        for m in moves {
            println!("{}", m);
        }
    }

    #[test]
    fn more_testing() {
        let board_string = concat!("    \n", "♟♟  \n", " ♘♙ \n", "   ♜");

        let board = GameBoard::from_string(4, 4, board_string).unwrap();

        let rook = board.check_space(2, 1).unwrap();

        let moves = rook.get_legal_moves(2, 1, &board, None);

        for m in moves {
            println!("{}", m);
        }
    }
}
