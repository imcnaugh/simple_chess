use std::collections::HashMap;

#[derive(Debug)]
pub struct Board {
    spaces: HashMap<String, String>,
}

impl Board {
    pub fn new(height: usize, width: usize) -> Board {

        let mut spaces = HashMap::new();

        for h in 0..height {
            for w in 0..width {
                let column_name = get_colum_name(w);
                let id = format!("{}{}", column_name, h+1);
                spaces.insert(id, String::from(""));
            }
        }

        Board {
            spaces
        }
    }
}

fn get_colum_name(col_number: usize) -> char {
    let chars: Vec<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect();
    let index = col_number % chars.len();
    chars[index]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_board() {
        let new_board = Board::new(8,8);

        println!("{:?}", new_board);
    }
}
