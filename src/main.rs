use chess::Game;

fn main() {
    let game = Game::new_chess_game();

    println!("{}", game.board);
}
