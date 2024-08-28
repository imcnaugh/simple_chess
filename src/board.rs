use std::collections::HashMap;
use std::fmt;

use crate::base_converter;

#[derive(Debug)]
pub struct Board {
    spaces: HashMap<String, Option<String>>,
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
                spaces.insert(id.clone(), None);
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
        let pattern = "+-";
        let mut r = pattern.repeat((self.width + 1) as usize);
        r.pop();

        let mut response = String::new();
        response.push_str(&r);
        response.push('\n');

        let print_row = |row_num| -> String {
            let divider = "|";

            let mut r = String::new();
            r.push_str(divider);

            for col in 1..=self.width {
                let key = format!(
                    "{}{}",
                    base_converter::get_column_name_from_index(col),
                    row_num
                );

                let s = &self
                    .spaces
                    .get(&key)
                    .unwrap_or(&None)
                    .clone()
                    .unwrap_or(String::from(" "));

                r.push_str(&s);
                r.push_str(divider);
            }

            r.to_string()
        };

        for h in (1..=self.height).rev() {
            let row = print_row(h);
            response.push_str(&row);
            response.push('\n');
            response.push_str(&r);
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
    }
}
