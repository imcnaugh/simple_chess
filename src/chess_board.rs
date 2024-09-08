use crate::chess_piece::ChessPiece;
use std::fmt;

pub struct ChessBoard<'a> {
    board: [[Option<&'a ChessPiece>; 8]; 8],
}

impl<'a> ChessBoard<'a> {
    pub fn new() -> ChessBoard<'a> {
        ChessBoard {
            board: [[None; 8]; 8],
        }
    }

    pub fn place_piece(&mut self, piece: &'a ChessPiece, x: usize, y: usize) {
        self.board[y][x] = Some(piece);
    }

    pub fn remove_piece(&mut self, x: usize, y: usize) -> Option<&ChessPiece> {
        self.board[y][x].take()
    }
}

impl<'a> fmt::Display for ChessBoard<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut board_string = String::new();
        board_string.push_str("Printing board!\n");
        for row in self.board.iter().rev() {
            for square in row.iter() {
                match square {
                    Some(piece) => board_string.push_str(&format!("{}", piece)),
                    None => board_string.push_str(" "),
                }
            }
            board_string.push('\n');
        }
        write!(f, "{}", board_string)
    }
}
