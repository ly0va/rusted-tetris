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
                Key::Char('a') | Key::Left  => game.shift(Direction::Left),
                Key::Char('d') | Key::Right => game.shift(Direction::Right),
                Key::Char('w') | Key::Up    => game.turn(),
                Key::Char('s') | Key::Down  => game.hard_drop(),
                Key::Char('q') | Key::Ctrl('c') => break,
                Key::Char(' ') => game.toggle_pause(),
                _ => ()
            }
            Err(_) => {
                println!("Error!\r");
                break;
            }
        }
        game.render();
    }
}
