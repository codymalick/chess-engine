mod board;
mod fen;
mod game;

fn main() {
    let mut game = game::Game::new();
    game.play()
}
