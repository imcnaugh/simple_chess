use crate::chess_piece::ChessPiece;
use std::fmt;

pub struct ChessBoard {
    board: [[Option<ChessPiece>; 8]; 8],
}

impl ChessBoard {
    pub fn new() -> ChessBoard {
        ChessBoard {
            board: Default::default(),
        }
    }

    pub fn place_piece(&mut self, piece: ChessPiece, x: usize, y: usize) {
        self.board[y][x] = Some(piece);
    }

    pub fn remove_piece(&mut self, x: usize, y: usize) -> Option<ChessPiece> {
        self.board[y][x].take()
    }
}

impl fmt::Display for ChessBoard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut board_string = String::new();
        for (x, row) in self.board.iter().enumerate().rev() {
            for (y, square) in row.iter().enumerate() {
                let inner_char = match square {
                    Some(piece) => format!("{}", piece),
                    None => " ".to_string(),
                };
                let square_color = if (x + y) % 2 != 0 {
                    "\x1b[107m"
                } else {
                    "\x1b[100m"
                };
                board_string.push_str(format!("{} {} \x1b[0m", square_color, inner_char).as_str());
            }
            board_string.push('\n');
        }
        write!(f, "{}", board_string)
    }
}
