mod board;
mod game;
use game::Game;

fn main() {
    let game: Game = Game::new();
    for r#move in game.possible_moves() {
        print!("{}\n", r#move.to_string());
    }
    print!("{}", game.to_string());
}
