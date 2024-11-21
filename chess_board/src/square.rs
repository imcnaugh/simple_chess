use crate::color::Color;
use crate::piece::Piece;
use core::fmt;
use std::fmt::Formatter;

/// Converts a given column and row to a chess-style coordinate string.
///
/// The function works by repeatedly dividing the column index by 26 to
/// convert it to a base-26 representation, with 'a' representing 1, 'b'
/// representing 2, and so on. This representation is reversed and
/// combined with the row (incremented by 1) to produce the final coordinate.
///
/// # Arguments
///
/// * `column` - A `usize` representing the column index, where 0 corresponds to 'a'.
/// * `row` - A `usize` representing the row index, which is zero-based.
///
/// # Returns
///
/// A `String` containing the coordinate in chess notation.
///
/// # Examples
///
/// ```
/// use chess_board::get_name_from_row_and_col;
/// let coordinate = get_name_from_row_and_col(0, 0);
/// assert_eq!(coordinate, "a1");
///
/// let coordinate = get_name_from_row_and_col(25, 1);
/// assert_eq!(coordinate, "z2");
///
/// let coordinate = get_name_from_row_and_col(26, 0);
/// assert_eq!(coordinate, "aa1");
/// ```
pub fn get_name_from_row_and_col(column: usize, row: usize) -> String {
    let mut col_id = String::new();
    let mut remainder = column;

    loop {
        let r = remainder % 26;
        let c = (r as u8 + b'a') as char;
        col_id.push(c);
        remainder /= 26;
        if remainder == 0 {
            break;
        }
        remainder -= 1;
    }

    col_id = col_id.chars().rev().collect();

    format!("{}{}", col_id, row + 1)
}

/// Converts a chess-style coordinate string to a given column and row.
///
/// The function works by separating the alphabetic characters (columns)
/// from the numeric characters (rows) in the input string. Once split,
/// it converts the alphabetic column to a base-26 number and adjusts it
/// to be zero-based. It also converts the numeric row to be zero-based.
///
/// # Arguments
///
/// * `name` - A `&str` representing the coordinate in chess notation,
///            with alphabetic characters for the column and numeric
///            characters for the row. Examples include "a1", "b2", "z2", etc.
///
/// # Returns
///
/// A `Result<(usize, usize), &str>` containing a tuple with the column
/// and row indices as `usize` if the input is valid, or an error string
/// if the input is invalid.
///
/// # Errors
///
/// This function returns an error if the input contains any invalid
/// characters (non-alphabetic characters in the column part or
/// non-numeric characters in the row part), or if the column or
/// row parts are empty.
///
/// # Examples
///
/// ```
/// use chess_board::get_column_and_row_from_name;
///
/// let (column, row) = get_column_and_row_from_name("a1").unwrap();
/// assert_eq!(column, 0);
/// assert_eq!(row, 0);
///
/// let (column, row) = get_column_and_row_from_name("b2").unwrap();
/// assert_eq!(column, 1);
/// assert_eq!(row, 1);
///
/// let (column, row) = get_column_and_row_from_name("ab1").unwrap();
/// assert_eq!(column, 27);
/// assert_eq!(row, 0);
///
/// let (column, row) = get_column_and_row_from_name("zzz100").unwrap();
/// assert_eq!(column, 18277);
/// assert_eq!(row, 99);
/// ```
///
/// # Panics
///
/// The function will panic if it fails to parse the row part into a `usize`.
pub fn get_column_and_row_from_name(name: &str) -> Result<(usize, usize), &str> {
    let mut col_as_string = String::new();
    let mut row_as_string = String::new();
    let mut finding_col = true;

    for c in name.chars() {
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
    if col_as_string.is_empty() || row_as_string.is_empty() {
        return Err("Invalid input");
    }

    let mut column: usize = 0;
    for (index, c) in col_as_string.chars().enumerate() {
        let base_26 = c as usize - 'a' as usize + 1;
        let s = 26_usize.pow((col_as_string.len() - index - 1) as u32);

        column += s * base_26;
    }

    let row: usize = row_as_string.parse().unwrap();
    Ok((column - 1, row - 1))
}

/// Represents a square on a chess board.
///
/// The `Square` struct holds the column and row indices, the color of the square,
/// and an optional piece that might occupy the square. The color is determined based
/// on the column and row indices.
///
/// # Type Parameters
///
/// * `P` - A type that implements the `Piece` trait, representing the type piece that can be placed on the square.
///
/// # Fields
///
/// * `column` - The zero-based column index of the square.
/// * `row` - The zero-based row index of the square.
/// * `color` - The color of the square, which can be either white or black.
/// * `piece` - An optional field that holds a piece of type `P` if present on the square.
/// ```
pub struct Square<> {
    column: usize,
    row: usize,
    color: Color,
    piece: Option<Box<dyn Piece>>,
}

impl Square {
    pub fn build(column: usize, row: usize) -> Self {
        let color = if (column + row) % 2 == 1 {
            Color::White
        } else {
            Color::Black
        };
        Square {
            color,
            piece: None,
            column,
            row,
        }
    }

    pub fn place_piece(&mut self, piece: Box<dyn Piece>) {
        self.piece = Some(piece);
    }

    pub fn get_piece(&self) -> Option<&Box<dyn Piece>> {
        self.piece.as_ref()
    }

    pub fn clear_piece(&mut self) -> Option<Box<dyn Piece>> {
        self.piece.take()
    }

    pub fn get_column(&self) -> usize {
        self.column
    }

    pub fn get_row(&self) -> usize {
        self.row
    }

    pub fn get_color(&self) -> Color {
        self.color
    }

    pub fn get_name(&self) -> String {
        get_name_from_row_and_col(self.column, self.row)
    }
}

impl fmt::Display for Square {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let square_color = match &self.color {
            Color::White => "\x1b[100m",
            Color::Black => "",
        };
        let inner_char = match &self.piece {
            Some(piece) => piece.get_char_representation(),
            None => ' ',
        };
        write!(f, "{} {} \x1b[0m", square_color, inner_char)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn col_row_turn_into_id() {
        let square_a1 = get_name_from_row_and_col(0, 0);
        assert_eq!("a1", format!("{square_a1}"));

        let square_z2 = get_name_from_row_and_col(25, 1);
        assert_eq!("z2", format!("{square_z2}"));

        let square_aa1 = get_name_from_row_and_col(26, 0);
        assert_eq!("aa1", format!("{square_aa1}"));

        let square_ab1 = get_name_from_row_and_col(27, 0);
        assert_eq!("ab1", format!("{square_ab1}"));

        let square_zzz100 = get_name_from_row_and_col(18277, 99);
        assert_eq!("zzz100", format!("{square_zzz100}"));
    }

    #[test]
    fn string_to_id() {
        let (a1_column, a1_row) = get_column_and_row_from_name("a1").unwrap();
        assert_eq!(0, a1_column);
        assert_eq!(0, a1_row);

        let (b2_column, b2_row) = get_column_and_row_from_name("b2").unwrap();
        assert_eq!(1, b2_column);
        assert_eq!(1, b2_row);

        let (ab1_column, ab1_row) = get_column_and_row_from_name("ab1").unwrap();
        assert_eq!(27, ab1_column);
        assert_eq!(0, ab1_row);

        let (zzz100_column, zzz100_row) = get_column_and_row_from_name("zzz100").unwrap();
        assert_eq!(18277, zzz100_column);
        assert_eq!(99, zzz100_row);
    }
}
