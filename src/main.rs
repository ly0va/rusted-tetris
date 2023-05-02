mod ai;
mod controls;
mod events;
mod game;
mod tetromino;

use controls::{Action, GameController};
use events::Event;
use std::error::Error;
use std::time::Duration;

use termion::event::Key;
use tetromino::Direction;

fn main() -> Result<(), Box<dyn Error>> {
    pretty_env_logger::init();
    match std::env::args().nth(1) {
        None => play(),
        Some(cmd) => {
            if cmd == "evolve" {
                evolve();
                Ok(())
            } else if cmd == "bot" {
                bot()
            } else {
                Err("unknown command".into())
            }
        }
    }
}

fn bot() -> Result<(), Box<dyn Error>> {
    let mut controller = GameController::new()?;
    let event = events::receiver();
    let bot = ai::Population::single(
        ai::DNA(vec![
            -0.8909047183906003,
            -0.08339877358128837,
            0.39718230475939464,
            -0.0781154639711134,
            -0.18835503282125332,
        ]),
        vec![
            Box::new(ai::genes::TotalHeight),
            // Box::new(ai::genes::MaxHeight),
            Box::new(ai::genes::LinesCleared),
            Box::new(ai::genes::Holes),
            Box::new(ai::genes::Bumpiness),
        ],
    );
    while !controller.game.over {
        while let Ok(event) = event.try_recv() {
            if let Event::Input(Key::Ctrl('c')) = event {
                return Ok(());
            }
        }
        let (shifts, rotations) = bot.best_actions(0, &controller.game);
        for _ in 0..10 {
            controller.game.shift(Direction::Left);
            controller.render()?;
        }
        for _ in 0..shifts {
            controller.game.shift(Direction::Right);
            controller.render()?;
            std::thread::sleep(Duration::from_millis(100));
        }
        for _ in 0..rotations {
            controller.game.rotate();
            controller.render()?;
            std::thread::sleep(Duration::from_millis(100));
        }
        controller.game.hard_drop();
        controller.render()?;
        std::thread::sleep(Duration::from_millis(100));
        controller.game.tick();
        controller.render()?;
        std::thread::sleep(Duration::from_millis(100));
    }
    Ok(())
}

fn evolve() {
    let mut population = ai::Population::new(
        1000,
        vec![
            Box::new(ai::genes::TotalHeight),
            // Box::new(ai::genes::MaxHeight),
            Box::new(ai::genes::LinesCleared),
            Box::new(ai::genes::Holes),
            Box::new(ai::genes::Bumpiness),
        ],
        18,
    );

    population.evolve(100);
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
