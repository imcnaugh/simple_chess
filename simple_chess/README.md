# Simple Chess

This package provides a comprehensive implementation of the game of chess. It includes all the necessary components to
simulate a chess game, and offers functionalities for game setup, move validation, and game state evaluation. This
package is suitable for both hobbyists and developers looking to integrate a chess engine into their applications.

## Features

- Full implementation of chess rules
- Move generation and validation
- Game state management
- Support for custom board setups
- Undo and redo moves
- Serializing/Deserializing via [Forsyth–Edwards Notation](https://en.wikipedia.org/wiki/Forsyth%E2%80%93Edwards_Notation)

## Usage

Below is a basic usage example showcasing how to create a new chess game and make a move:

```rust
use chess::chess_game_state_analyzer::GameState;
use chess::ChessGame;

fn main() {
    let mut game = ChessGame::new();
    let mut state = game.get_game_state();
    match state {
        GameState::InProgress { legal_moves, turn } => {
            println!("play on, Its {:?}'s turn.", turn);
            state = game.make_move(legal_moves[0])
        },
        _ => ()
    };
    println!("{}", game.get_board());
}
```

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE.txt) file for more details.