mod controls;
mod events;
mod game;
mod tetromino;

use controls::{Action, GameControls};
use events::Event;
use std::error::Error;
use termion::event::Key;
use tetromino::Direction;

fn main() -> Result<(), Box<dyn Error>> {
    let mut controls = GameControls::new()?;
    let event = events::receiver();
    while !controls.game.over {
        match event.recv()? {
            Event::Tick => controls.send(Action::Tick),
            Event::Input(key) => match key {
                Key::Char('a') | Key::Left => controls.send(Action::Shift(Direction::Left)),
                Key::Char('d') | Key::Right => controls.send(Action::Shift(Direction::Right)),
                Key::Char('w') | Key::Up => controls.send(Action::Turn),
                Key::Char('s') | Key::Down => controls.send(Action::HardDrop),
                Key::Char('q') | Key::Ctrl('c') => break,
                Key::Char(' ') => controls.toggle_pause(),
                _ => (),
            },
        }
        controls.render()?;
    }
    Ok(())
}
