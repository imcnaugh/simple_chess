use std::collections::HashMap;
use std::fmt;

use crate::{BoardSquare, SquareColor, Direction};

use crate::base_converter;

pub struct Board {
    spaces: HashMap<String, BoardSquare>,
    height: u8,
    width: u8,
}

impl Board {
    pub fn new<'a>(height: u8, width: u8) -> Board {
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

                let square = BoardSquare::new(id.clone(), square_color);
                spaces.insert(id.clone(), square);
            }
        }

        Board {
            spaces,
            height,
            width,
        }
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
