use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;
use std::cell::RefCell;

use crate::{BoardSquare, SquareColor, Direction};

use crate::base_converter;

pub struct Board {
    spaces: HashMap<String, Rc<RefCell<BoardSquare>>>,
    height: u8,
    width: u8,
}

impl Board {
    pub fn new(height: u8, width: u8) -> Board {
        let mut spaces = HashMap::new();

        for h in 1..=height {
            for w in 1..=width {
                let column_name = base_converter::get_column_name_from_index(w);
                let id = format!("{}{}", column_name, h);

                let square_color = if w % 2 == (if h % 2 == 0 {1} else {0}) {
                    SquareColor::White
                } else {
                    SquareColor::Black
                };

                let square = Rc::new(RefCell::new(BoardSquare::new(id.clone(), square_color)));
                spaces.insert(id.clone(), square);
            }
        }

        for (id, space) in &spaces {
            let (col, row) = Self::split_id_to_row_col(id);
            let curent_col = base_converter::get_column_name_from_index(col);
            let left_col = base_converter::get_column_name_from_index(col - 1);
            let right_col = base_converter::get_column_name_from_index(col + 1);
            let top_row = row + 1;
            let bottom_row = row - 1;

            let set_space = |id: &str, direction: Direction| {
                if let Some(neighbor) = spaces.get(id) {
                    space.borrow_mut().add_neighbor(direction, neighbor);
                }
            };

            set_space(&format!("{curent_col}{top_row}"), Direction::North);
            set_space(&format!("{right_col}{top_row}"), Direction::NorthEast);
            set_space(&format!("{right_col}{row}"), Direction::East);
            set_space(&format!("{right_col}{bottom_row}"), Direction::SouthEast);
            set_space(&format!("{curent_col}{bottom_row}"), Direction::South);
            set_space(&format!("{left_col}{bottom_row}"), Direction::SouthWest);
            set_space(&format!("{left_col}{row}"), Direction::West);
            set_space(&format!("{left_col}{top_row}"), Direction::NorthWest);
        }

        Board {
            spaces,
            height,
            width,
        }
    }


    fn split_id_to_row_col(s: &str) -> (u8, u8) {
        let mut row = String::new();
        let mut col = String::new();
        let mut found_digit = false;

        for c in s.chars() {
            if !found_digit && c.is_ascii_digit() {
                found_digit = true;
            }

            if found_digit {
                col.push(c);
            } else {
                row.push(c);
            }
        }

        let col: u8 = col.parse().unwrap();
        let row = base_converter::get_index_from_column_name(&row);

        (row, col)
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut response = String::new();

        let print_row = |row_num| -> String {
            let mut r = String::new();

            for col in 1..=self.width {
                let key = format!(
                    "{}{}",
                    base_converter::get_column_name_from_index(col),
                    row_num
                );

                let s = &self
                    .spaces
                    .get(&key)
                    .unwrap()
                    .borrow()
                    .to_string();

                r.push_str(s);
            }

            r.to_string()
        };

        for h in (1..=self.height).rev() {
            let row = print_row(h);
            response.push_str(&row);
            response.push('\n');
        }

        write!(f, "{}", response)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_board() {
        let new_board = Board::new(8, 8);

        for col in 'a'..='h' {
            for row in '1'..='8' {
                let key = format!("{}{}", col, row);
                assert!(new_board.spaces.contains_key(&key))
            }
        }

        assert!(!new_board.spaces.contains_key("i1"));
        assert!(!new_board.spaces.contains_key("a9"));

        let a1 = new_board.spaces.get("a1").unwrap();
        let possible = a1.borrow().neighbors.get(&Direction::North).unwrap().upgrade();
        let idk = possible.unwrap().borrow().name.clone();
        assert_eq!("a2", idk);
    }
}
