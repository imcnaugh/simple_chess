use chess::chess_piece::{ChessPiece, PieceType};
use chess::{ChessBoard, Color};

fn main() {
    println!("Hello, world!");

    let white_pieces = create_piece_set(Color::White);
    let black_pieces = create_piece_set(Color::Black);

    let mut board = ChessBoard::new();

    let mut board = place_pieces_on_board(&mut board, &white_pieces, &black_pieces);

    &board.remove_piece(0, 1);

    println!("{}", &board);
}

fn place_pieces_on_board<'a>(
    board: &'a mut ChessBoard<'a>,
    white_pieces: &'a Vec<ChessPiece>,
    black_pieces: &'a Vec<ChessPiece>,
) -> &'a mut ChessBoard<'a> {
    let mut white_pawns = vec![0, 1, 2, 3, 4, 5, 6, 7].into_iter();
    let mut black_pawns = vec![0,1,2,3,4,5,6,7].into_iter();
    let mut white_rooks = vec![0, 7].into_iter();
    let mut black_rooks = vec![0, 7].into_iter();
    let mut white_knights = vec![1, 6].into_iter();
    let mut black_knights = vec![1, 6].into_iter();
    let mut white_bishops = vec![2, 5].into_iter();
    let mut black_bishops = vec![2, 5].into_iter();

    for piece in white_pieces {
        match piece.get_piece_type() {
            PieceType::Pawn => {
                board.place_piece(piece, white_pawns.next().unwrap(), 1);
            },
            PieceType::Rook => {
                board.place_piece(piece, white_rooks.next().unwrap(), 0);
            },
            PieceType::Knight => {
                board.place_piece(piece, white_knights.next().unwrap(), 0);
            },
            PieceType::Bishop => {
                board.place_piece(piece, white_bishops.next().unwrap(), 0);
            },
            PieceType::Queen => {
                board.place_piece(piece, 3, 0);
            },
            PieceType::King => {
                board.place_piece(piece, 4, 0);
            },
        }
    }

    for piece in black_pieces {
        match piece.get_piece_type() {
            PieceType::Pawn => {
                board.place_piece(piece, black_pawns.next().unwrap(), 6);
            },
            PieceType::Rook => {
                board.place_piece(piece, black_rooks.next().unwrap(), 7);
            },
            PieceType::Knight => {
                board.place_piece(piece, black_knights.next().unwrap(), 7);
            },
            PieceType::Bishop => {
                board.place_piece(piece, black_bishops.next().unwrap(), 7);
            },
            PieceType::Queen => {
                board.place_piece(piece, 3, 7);
            },
            PieceType::King => {
                board.place_piece(piece, 4, 7);
            },
        }
    }

    board
}

fn create_piece_set(color: Color) -> Vec<ChessPiece> {
    let mut pieces = Vec::new();
    for _ in 0..8 {
        pieces.push(ChessPiece::new(color, PieceType::Pawn));
    }
    for _ in 0..2 {
        pieces.push(ChessPiece::new(color, PieceType::Rook));
        pieces.push(ChessPiece::new(color, PieceType::Knight));
        pieces.push(ChessPiece::new(color, PieceType::Bishop));
    }
    pieces.push(ChessPiece::new(color, PieceType::Queen));
    pieces.push(ChessPiece::new(color, PieceType::King));
    pieces
}
