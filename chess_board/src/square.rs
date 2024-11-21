use core::fmt;
use std::fmt::Formatter;
use crate::color::Color;
use crate::piece::Piece;

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

#[derive(Copy, Clone)]
pub struct Square<P: Piece> {
    column: usize,
    row: usize,
    color: Color,
    piece: Option<P>,
}

impl<P: Piece> Square<P> {
    pub fn build(column: usize, row: usize) -> Self {
        let color = if (column + row) % 2 == 1 {
            Color::White
        } else {
            Color::Black
        };
        Square { color, piece: None, column, row }
    }

    pub fn place_piece(&mut self, piece: P) {
        self.piece = Some(piece);
    }

    pub fn get_piece(&self) -> Option<&P> {
        self.piece.as_ref()
    }

    pub fn clear_piece(&mut self) -> Option<P> {
        self.piece.take()
    }

    pub fn get_column(&self) -> usize {
        self.column
    }

    pub fn get_row(&self) -> usize {
        self.row
    }

    pub fn get_name(&self) -> String {
        get_name_from_row_and_col(self.column, self.row)
    }
}

impl<P: Piece> fmt::Display for Square<P> {
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
