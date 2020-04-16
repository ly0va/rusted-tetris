use console::*;

mod game;
use game::Game;

mod events;
use events::Event;

fn main() {
    let mut game = Game::new();
    let event = events::receiver();
    loop {
        match event.recv() {
            Ok(Event::Tick) => game.shift(1, 0),
            Ok(Event::Input(key)) => match key {
                Key::ArrowLeft => game.shift(0, -1),
                Key::ArrowRight => game.shift(0, 1),
                _ => ()
            }
            _ => println!("Error!\r")
        }
        game.render();
    }
}
