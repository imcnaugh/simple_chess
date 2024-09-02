use std::fmt;
use crate::{BoardSquare, SquareColor, Direction, Graph};
use crate::base_converter;

pub struct Board {
    spaces: Graph<String, BoardSquare, Direction>,
    height: u8,
    width: u8,
}

impl Board {
    pub fn new<'a>(height: u8, width: u8) -> Board {
        let mut spaces = Graph::new();

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
                spaces.add_node(id.clone(), square);
            }
        }

        for h in 1..=height {
            for w in 1..=width {
                let column_name = base_converter::get_column_name_from_index(w);
                let id = format!("{}{}", column_name, h);

                if h < height {
                    let north = format!("{}{}", column_name, h + 1);
                    spaces.add_edge(id.clone(), north, Direction::North);
                }

                if h > 1 {
                    let south = format!("{}{}", column_name, h - 1);
                    spaces.add_edge(id.clone(), south, Direction::South);
                }

                if w < width {
                    let east = format!("{}{}", base_converter::get_column_name_from_index(w + 1), h);
                    spaces.add_edge(id.clone(), east, Direction::East);
                }

                if w > 1 {
                    let west = format!("{}{}", base_converter::get_column_name_from_index(w - 1), h);
                    spaces.add_edge(id.clone(), west, Direction::West);
                }
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
                    .get_node(key)
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
                let square = new_board.spaces.get_node(key.clone()).unwrap();
                assert_eq!(key, square.name);
            }
        }

        // assert!(new_board.spaces.get_node(String::from("i1")));
        // assert!(new_board.spaces.get_node(String::from("a9")));

        let a1 = new_board.spaces.get_node(String::from("a1")).unwrap();
        let possible = new_board.spaces.get_edge(String::from("a1"), Direction::North);
    }
}
