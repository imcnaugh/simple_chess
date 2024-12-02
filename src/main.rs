use chess_game::ChessGame;

fn main() {
    let game = ChessGame::new();
    println!("{}", game.get_board());
}
