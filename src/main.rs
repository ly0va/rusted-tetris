mod events;
mod game;
mod tetromino;

use events::Event;
use game::Game;
use std::error::Error;
use termion::event::Key;
use tetromino::Direction;

fn main() -> Result<(), Box<dyn Error>> {
    let mut game = Game::new()?;
    let event = events::receiver();
    while !game.over {
        match event.recv()? {
            Event::Tick => game.tick(),
            Event::Input(key) => match key {
                Key::Char('a') | Key::Left => game.shift(Direction::Left),
                Key::Char('d') | Key::Right => game.shift(Direction::Right),
                Key::Char('w') | Key::Up => game.turn(),
                Key::Char('s') | Key::Down => game.hard_drop(),
                Key::Char('q') | Key::Ctrl('c') => break,
                Key::Char(' ') => game.toggle_pause(),
                _ => (),
            },
        }
        game.render()?;
    }
    Ok(())
}
