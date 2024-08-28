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

        for h in 1..(height + 1) {
            for w in 1..(width + 1) {
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

    fn print_row(&self, row_num: u8) -> String {
        let divider = "|";

        let mut r = String::new();
        r.push_str(divider);

        for col in 1..(self.width + 1) {
            let key = format!(
                "{}{}",
                base_converter::get_column_name_from_index(col),
                row_num
            );

            let s = self
                .spaces
                .get(&key)
                .unwrap_or(&None)
                .clone()
                .unwrap_or(String::from(" "));

            r.push_str(&s);
            r.push_str(divider);
        }

        r.to_string()
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

        for h in (1..(self.height + 1)).rev() {
            let row = Board::print_row(self, h);
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

        println!("{:?}", new_board);
    }
}
