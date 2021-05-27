mod events;
mod game;
mod tetromino;

use events::Event;
use game::GameControls;
use std::error::Error;
use termion::event::Key;
use tetromino::Direction;

fn main() -> Result<(), Box<dyn Error>> {
    let mut controls = GameControls::new()?;
    let event = events::receiver();
    while !controls.game.over {
        match event.recv()? {
            Event::Tick => controls.tick(),
            Event::Input(key) => match key {
                Key::Char('a') | Key::Left => controls.shift(Direction::Left),
                Key::Char('d') | Key::Right => controls.shift(Direction::Right),
                Key::Char('w') | Key::Up => controls.turn(),
                Key::Char('s') | Key::Down => controls.hard_drop(),
                Key::Char('q') | Key::Ctrl('c') => break,
                Key::Char(' ') => controls.toggle_pause(),
                _ => (),
            },
        }
        controls.render()?;
    }
    Ok(())
}
