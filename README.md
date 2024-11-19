# Rust Chess

## TODO
 - Chess notation for moves
   - [UCI](https://en.wikipedia.org/wiki/Universal_Chess_Interface) https://www.chessprogramming.org/UCI
   - [Algebraic notation](https://en.wikipedia.org/wiki/Algebraic_notation_(chess)#Long_algebraic_notation)
   - [PGN](https://en.wikipedia.org/wiki/Portable_Game_Notation)
 - [Forsyth–Edwards Notation](https://en.wikipedia.org/wiki/Forsyth%E2%80%93Edwards_Notation)
   - Create game
 - AI for opponent

## Thoughts

### Encoding the state of a chess board

Let's say we combine the bits so its {square state}{color}
so a Black Bishop would be 1001,

and 4 bits could represent a square

|Square State |Bit representation |
| ----------- | ----------------- |
|Empty        |000                |
|Pawn         |001                |
|Rook         |010                |
|Knight       |011                |
|Bishop       |100                |
|King         |101                |
|Queen        |110                |
|Empty        |111                |

|Color |Bit  |
| ---- | --- |
|White |0    |
|Black |1    |

with 4 bits per square, and 64 squares, we get 256 bits. We could hold the entire state of the board in 2 u128 ints, or displayed as a base 64 encoded string of 43 chars with 1 padding.

From this we could keep the view of a board in a hash map and use that for checking for repetition draws. or condensed transmission of the board state.