use crate::chess_piece::ChessPiece;
use std::fmt;

pub struct ChessBoard {
    board: Vec<Vec<Option<ChessPiece>>>,
    pub width: usize,
    pub height: usize,
}

impl ChessBoard {
    pub fn new(width: usize, height: usize) -> ChessBoard {
        ChessBoard {
            board: ChessBoard::generate_board(width, height),
            width,
            height,
        }
    }

    pub fn generate_chess_board() -> ChessBoard {
        let board_width = 8;
        let board_height = 8;

        ChessBoard {
            board: ChessBoard::generate_board(board_width, board_height),
            width: board_width,
            height: board_height,
        }
    }

    fn generate_board(width: usize, height: usize) -> Vec<Vec<Option<ChessPiece>>> {
        let mut board = Vec::new();
        for _ in 0..width {
            let mut row = Vec::new();
            for _ in 0..height {
                row.push(None);
            }
            board.push(row);
        }
        board
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
