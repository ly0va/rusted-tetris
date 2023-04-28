mod ai;
mod controls;
mod events;
mod game;
mod tetromino;

use controls::{Action, GameController};
use events::Event;
use std::error::Error;
use termion::event::Key;
use tetromino::Direction;

fn main() -> Result<(), Box<dyn Error>> {
    pretty_env_logger::init();
    evolve();
    Ok(())
    // play()
}

fn evolve() {
    let mut population = ai::Population::new_random(
        10000,
        vec![
            Box::new(ai::genes::MaxHeight),
            Box::new(ai::genes::Holes),
            Box::new(ai::genes::Bumpiness),
        ],
    );

    population.evolve(100);
    println!("{:?}", population.champion());
}

// TODO: use anyhow for errors
fn play() -> Result<(), Box<dyn Error>> {
    let mut controller = GameController::new()?;
    let event = events::receiver();
    while !controller.game.over {
        match event.recv()? {
            Event::Tick => controller.send(Action::Tick),
            Event::Input(key) => match key {
                Key::Char('a') | Key::Left => controller.send(Action::Shift(Direction::Left)),
                Key::Char('d') | Key::Right => controller.send(Action::Shift(Direction::Right)),
                Key::Char('w') | Key::Up => controller.send(Action::Turn),
                Key::Char('s') | Key::Down => controller.send(Action::HardDrop),
                Key::Char('q') | Key::Ctrl('c') => break,
                Key::Char(' ') => controller.toggle_pause(),
                _ => (),
            },
        }
        controller.render()?;
    }
    Ok(())
}
