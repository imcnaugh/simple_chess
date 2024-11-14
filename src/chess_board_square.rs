use core::fmt;
use std::fmt::{format, Display, Formatter};
use crate::{ChessPiece, Color};

#[derive(Copy, Clone)]
pub struct Square {
    id: SquareId,
    color: Color,
    piece: Option<ChessPiece>,
}

impl Square {
    pub fn build(column: usize, row: usize) -> Self {
        let color = if (column + row) % 2 == 1 { Color::White } else { Color::Black };
        Square {
            id: SquareId::build(column, row),
            color,
            piece: None,
        }
    }

    pub fn get_id(&self) -> &SquareId {
        &self.id
    }

    pub fn place_piece(&mut self, piece: ChessPiece) {
        self.piece = Some(piece);
    }

    pub fn get_piece(&self) -> Option<&ChessPiece> {
        self.piece.as_ref()
    }

    pub fn clear_piece(&mut self) -> Option<ChessPiece>{
        let piece = self.piece;
        self.piece = None;
        piece
    }
}

impl fmt::Display for Square {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let square_color = match self.color {
            Color::White => "",
            Color::Black => "\x1b[100m",
        };
        let inner_char = match self.piece {
            Some(piece) => format!("{piece}"),
            None => " ".to_string(),
        };
        write!(f, "{} {} \x1b[0m", square_color, inner_char)
    }
}

/// # Square Id
#[derive(PartialEq, Eq, Hash, Copy, Clone)]
pub struct SquareId {
    column: usize,
    row: usize,
}

impl SquareId {
    /// # Build
    ///
    /// Creates a new square id from a row and a column
    ///
    /// Column and row are usize, and can scale beyond the typical size of a chess board.
    pub fn build(column: usize, row: usize) -> Self {
        Self { column, row }
    }

    /// # From string
    ///
    /// Create a square id from a string
    pub fn from_string(s: &str) -> Result<Self, &str> {
        let mut col_as_string = String::new();
        let mut row_as_string = String::new();
        let mut finding_col = true;

        for c in s.chars() {
            if finding_col {
                if c.is_ascii_alphabetic() {
                    col_as_string.push(c);
                } else if c.is_ascii_digit() {
                    finding_col = false;
                    row_as_string.push(c);
                } else {
                    return Err("Invalid input");
                }
            } else if c.is_ascii_digit() {
                row_as_string.push(c);
            } else {
                return Err("Invalid input");
            }
        }

        let mut column: usize = 0;
        for (index, c) in col_as_string.chars().enumerate() {
            let base_26 = c as usize - 'a' as usize + 1;
            let s = 26_usize.pow((col_as_string.len() - index - 1) as u32);

            column += s * base_26;
        }

        let row: usize = row_as_string.parse().unwrap();
        Ok(Self{
            column: column - 1,
            row: row - 1
        })
    }

    pub fn get_column(&self) -> usize {
        self.column
    }

    pub fn get_row(&self) -> usize {
        self.row
    }
}

impl fmt::Display for SquareId {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut col_id = String::new();
        let mut remainder = self.column;

        loop {
            let r = remainder % 26;
            let c = (r as u8 + b'a') as char;
            col_id = format!("{}{}", c, col_id);
            remainder /= 26;
            if remainder == 0 {
                break;
            }
            remainder -= 1;
        }

        write!(f, "{}{}", col_id, self.row + 1)
    }
}

#[cfg(test)]
mod tests {
    use rand::Rng;
    use super::*;

    #[test]
    fn col_row_turn_into_id() {
        let square_a1 = SquareId::build(0, 0);
        assert_eq!(square_a1.get_column(), 0);
        assert_eq!(square_a1.get_row(), 0);
        assert_eq!("a1", format!("{square_a1}"));

        let square_z2 = SquareId::build(25, 1);
        assert_eq!("z2", format!("{square_z2}"));

        let square_aa1 = SquareId::build(26, 0);
        assert_eq!("aa1", format!("{square_aa1}"));

        let square_ab1 = SquareId::build(27, 0);
        assert_eq!("ab1", format!("{square_ab1}"));

        let square_zzz100 = SquareId::build(18277, 99);
        assert_eq!("zzz100", format!("{square_zzz100}"));
    }

    #[test]
    fn string_to_id() {
        let square_a1 = SquareId::from_string("a1").unwrap();
        assert_eq!(0, square_a1.get_column());
        assert_eq!(0, square_a1.get_row());

        let square_b2 = SquareId::from_string("b2").unwrap();
        assert_eq!(1, square_b2.get_column());
        assert_eq!(1, square_b2.get_row());

        let square_ab1 = SquareId::from_string("ab1").unwrap();
        assert_eq!(27, square_ab1.get_column());
        assert_eq!(0, square_ab1.get_row());

        let square_zzz100 = SquareId::from_string("zzz100").unwrap();
        assert_eq!(18277, square_zzz100.get_column());
        assert_eq!(99, square_zzz100.get_row());
    }

    #[test]
    fn board_squares_built_from_string_are_equal_to_squares_build_from_ints() {
        let random_col = rand::thread_rng().gen_range(0..1000);
        let random_row = rand::thread_rng().gen_range(0..1000);

        let square_id = SquareId::build(random_col, random_row);

        let square_id_from_string = SquareId::from_string(format!("{square_id}").as_str()).unwrap();

        assert_eq!(square_id.column, square_id_from_string.column);
        assert_eq!(square_id.row, square_id_from_string.row);
    }
}
