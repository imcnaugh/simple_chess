use crate::piece::ChessPiece;
use game_board::Board;

/// Encodes a simple_chess board into a binary vector representation.
///
/// # Arguments
///
/// * `board` - A reference to a `Board<ChessPiece>` representing the simple_chess board to be encoded.
///
/// # Returns
///
/// A `Vec<u8>` where each byte represents two simple_chess pieces on the board. The encoding is such that:
/// - The board is traversed from top to bottom, left to right.
/// - Each pair of consecutive spaces on the board is encoded into a single byte.
/// - The first piece in the pair is stored in the higher 4 bits of the byte, and the second piece in the lower 4 bits.
/// - If a space is empty, it is represented by `0b0000`.
/// - If a space has a piece on it, it uses the following table to generate a 4 byte code as `{piece_type}{color}`
///   |Square State |Bit representation |
///   | ----------- | ----------------- |
///   |Empty        |000                |
///   |Pawn         |001                |
///   |Rook         |010                |
///   |Knight       |011                |
///   |Bishop       |100                |
///   |King         |101                |
///   |Queen        |110                |
///
///   |Color |Bit  |
///   | ---- | --- |
///   |White |0    |
///   |Black |1    |
///
pub fn encode_board_as_binary(board: &Board<ChessPiece>) -> Vec<u8> {
    let mut encoded_board = Vec::new();

    let mut first = true;
    let mut current: u8 = 0;
    for row in (0..board.get_height()).rev() {
        for col in 0..board.get_width() {
            let binary = if let Some(piece) = board.get_piece_at_space(col, row) {
                piece.as_binary()
            } else {
                0b0000
            };

            current |= binary;
            if first {
                current <<= 4;
            } else {
                encoded_board.push(current);
                current = 0;
            }
            first = !first;
        }
    }

    encoded_board
}

#[cfg(test)]
mod tests {

    mod encoding_tests {
        use super::super::*;
        use crate::piece::PieceType::King;
        use crate::ChessGame;
        use crate::Color::Black;

        #[test]
        fn encode_starting_position() {
            let game = ChessGame::new();
            let encoded = encode_board_as_binary(game.get_board());

            assert_eq!(32, encoded.len());
            assert_eq!(0b01010111, encoded[0]);
            assert_eq!(0b10011101, encoded[1]);
            assert_eq!(0b10111001, encoded[2]);
            assert_eq!(0b01110101, encoded[3]);
            for i in 4..8 {
                assert_eq!(0b00110011, encoded[i]);
            }
            for i in 8..24 {
                assert_eq!(0b00000000, encoded[i]);
            }
            for i in 24..28 {
                assert_eq!(0b00100010, encoded[i]);
            }
            assert_eq!(0b01000110, encoded[28]);
            assert_eq!(0b10001100, encoded[29]);
            assert_eq!(0b10101000, encoded[30]);
            assert_eq!(0b01100100, encoded[31]);
        }

        #[test]
        fn encode_non_standard_board() {
            let mut board = Board::build(1, 3).unwrap();
            board.place_piece(ChessPiece::new(King, Black), 0, 1);

            let encoded = encode_board_as_binary(&board);

            assert_eq!(1, encoded.len());
            assert_eq!(0b00001011, encoded[0]);
        }
    }
}
