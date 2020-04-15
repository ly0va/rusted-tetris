use std::thread;
use std::time::Duration;

mod game;
use game::Game;

fn main() {
    let mut game = Game::new();
    for i in 0..15 {
        game.draw_piece(false);
        game.tetromino.shift(1, 0);
        game.draw_piece(true);
        game.render();
        thread::sleep(Duration::from_millis(500));
    }
}
