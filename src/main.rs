use console::*;

mod game;
use game::Game;
use game::tetromino::Direction;

mod events;
use events::Event;

fn main() {
    let mut game = Game::new();
    let event = events::receiver();
    loop {
        match event.recv() {
            Ok(Event::Tick) => game.shift(Direction::Down),
            Ok(Event::Input(key)) => match key {
                Key::ArrowLeft => game.shift(Direction::Left),
                Key::ArrowRight => game.shift(Direction::Right),
                _ => ()
            }
            _ => println!("Error!\r")
        }
        game.render();
    }
}
