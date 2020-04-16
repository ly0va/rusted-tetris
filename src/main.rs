#![allow(unused_must_use)]

mod game;
mod events;
use game::Game;
use game::tetromino::Direction;
use events::Event;
use termion::event::Key;

fn main() {
    let mut game = Game::new();
    let event = events::receiver();
    while !game.over {
        match event.recv() {
            Ok(Event::Tick) => game.tick(),
            Ok(Event::Input(key)) => match key {
                Key::Char('a') => game.shift(Direction::Left),
                Key::Char('d') => game.shift(Direction::Right),
                Key::Char('w') => game.turn(),
                Key::Char('s') => game.hard_drop(),
                Key::Char('q') | Key::Ctrl('c') => break,
                _ => ()
            }
            _ => println!("Error!\r")
        }
        game.render();
    }
}
