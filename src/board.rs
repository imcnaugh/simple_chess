use std::fmt;
use crate::{BoardSquare, SquareColor, Direction, Graph};
use crate::base_converter;
use crate::pieces::peice::{
    PieceColor,
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
};

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

                let mut add_edge = |from: String, to: String, direction: Direction| {
                    match spaces.get_node(to.clone()) {
                        Some(_) => spaces.add_edge(from, to, direction),
                        None => return,
                    }
                };

                add_edge(id.clone(), format!("{}{}", column_name, h + 1), Direction::North);
                add_edge(id.clone(), format!("{}{}", base_converter::get_column_name_from_index(w + 1), h + 1), Direction::NorthEast);
                add_edge(id.clone(), format!("{}{}", base_converter::get_column_name_from_index(w + 1), h), Direction::East);
                add_edge(id.clone(), format!("{}{}", base_converter::get_column_name_from_index(w + 1), h - 1), Direction::SouthEast);
                add_edge(id.clone(), format!("{}{}", column_name, h - 1), Direction::South);
                add_edge(id.clone(), format!("{}{}", base_converter::get_column_name_from_index(w - 1), h - 1), Direction::SouthWest);
                add_edge(id.clone(), format!("{}{}", base_converter::get_column_name_from_index(w - 1), h), Direction::West);
                add_edge(id.clone(), format!("{}{}", base_converter::get_column_name_from_index(w - 1), h + 1), Direction::NorthWest);
            }
        }

        spaces.get_node_mut(String::from("a1"))
            .unwrap()
            .set_piece(Box::new(Rook::new(PieceColor::White)));
        spaces.get_node_mut(String::from("b1"))
            .unwrap()
            .set_piece(Box::new(Knight::new(PieceColor::White)));
        spaces.get_node_mut(String::from("c1"))
            .unwrap()
            .set_piece(Box::new(Bishop::new(PieceColor::White)));
        spaces.get_node_mut(String::from("d1"))
            .unwrap()
            .set_piece(Box::new(Queen::new(PieceColor::White)));
        spaces.get_node_mut(String::from("e1"))
            .unwrap()
            .set_piece(Box::new(King::new(PieceColor::White)));
        spaces.get_node_mut(String::from("f1"))
            .unwrap()
            .set_piece(Box::new(Bishop::new(PieceColor::White)));
        spaces.get_node_mut(String::from("g1"))
            .unwrap()
            .set_piece(Box::new(Knight::new(PieceColor::White)));
        spaces.get_node_mut(String::from("h1"))
            .unwrap()
            .set_piece(Box::new(Rook::new(PieceColor::White)));

        for col in 'a'..='h' {
            let key = format!("{}2", col);
            spaces.get_node_mut(key)
                .unwrap()
                .set_piece(Box::new(Pawn::new(PieceColor::White)));
        }

        spaces.get_node_mut(String::from("a8"))
            .unwrap()
            .set_piece(Box::new(Rook::new(PieceColor::Black)));
        spaces.get_node_mut(String::from("b8"))
            .unwrap()
            .set_piece(Box::new(Knight::new(PieceColor::Black)));
        spaces.get_node_mut(String::from("c8"))
            .unwrap()
            .set_piece(Box::new(Bishop::new(PieceColor::Black)));
        spaces.get_node_mut(String::from("d8"))
            .unwrap()
            .set_piece(Box::new(Queen::new(PieceColor::Black)));
        spaces.get_node_mut(String::from("e8"))
            .unwrap()
            .set_piece(Box::new(King::new(PieceColor::Black)));
        spaces.get_node_mut(String::from("f8"))
            .unwrap()
            .set_piece(Box::new(Bishop::new(PieceColor::Black)));
        spaces.get_node_mut(String::from("g8"))
            .unwrap()
            .set_piece(Box::new(Knight::new(PieceColor::Black)));
        spaces.get_node_mut(String::from("h8"))
            .unwrap()
            .set_piece(Box::new(Rook::new(PieceColor::Black)));

        for col in 'a'..='h' {
            let key = format!("{}7", col);
            spaces.get_node_mut(key)
                .unwrap()
                .set_piece(Box::new(Pawn::new(PieceColor::Black)));
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

        assert!(new_board.spaces.get_node(String::from("i1")).is_none());
        assert!(new_board.spaces.get_node(String::from("a9")).is_none());

        let a1 = new_board.spaces.get_node(String::from("a1")).unwrap();
        let possible = new_board.spaces.get_edge(String::from("a1"), Direction::North);
        assert_eq!(possible, Some(&String::from("a2")));
    }
}
