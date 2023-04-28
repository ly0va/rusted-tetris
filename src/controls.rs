use crate::game::{StandardGame, HEIGHT, WIDTH};
use crate::tetromino::Direction;
use std::io::{self, Write};
use termion::{
    color::{self, Bg, Fg},
    cursor,
    raw::*,
};

pub struct GameController {
    pub game: StandardGame,
    pub pause: bool,
    out: RawTerminal<io::Stdout>,
}

pub enum Action {
    Turn,
    Shift(Direction),
    HardDrop,
    Tick,
}

impl GameController {
    pub fn new() -> io::Result<Self> {
        let mut stdout = io::stdout().into_raw_mode()?;
        write!(stdout, "{}{}", cursor::Hide, termion::clear::All)?;
        Ok(GameController {
            game: StandardGame::new(),
            pause: false,
            out: stdout,
        })
    }

    pub fn toggle_pause(&mut self) {
        self.pause = !self.pause;
    }

    pub fn render(&mut self) -> io::Result<()> {
        self.game.draw_piece(true);
        write!(self.out, "{}", cursor::Goto(1, 1))?;
        let wall = format!("{} {}", Bg(color::White), Bg(color::Reset));
        for i in 0..HEIGHT {
            let row = (0..WIDTH)
                .map(|j| format!("{}  {}", self.game.grid[i][j], Bg(color::Reset)))
                .collect::<String>();
            writeln!(self.out, "{}{}{}\r", wall, row, wall)?;
        }
        let bottom = (0..=WIDTH).map(|_| "  ").collect::<String>();
        write!(
            self.out,
            "{}{}{}",
            Bg(color::White),
            Fg(color::Black),
            bottom
        )?;
        writeln!(
            self.out,
            "{} Score: {}{}{}\r",
            cursor::Goto(1, 1 + HEIGHT as u16),
            self.game.score,
            Bg(color::Reset),
            Fg(color::Reset)
        )?;
        self.game.draw_piece(false);
        self.out.flush()
    }

    pub fn send(&mut self, action: Action) {
        if self.pause {
            return;
        }
        match action {
            Action::Turn => self.game.turn(),
            Action::Tick => self.game.tick(),
            Action::HardDrop => self.game.hard_drop(),
            Action::Shift(dir) => self.game.shift(dir),
        }
    }
}

impl Drop for GameController {
    #[allow(unused_must_use)]
    fn drop(&mut self) {
        write!(self.out, "{}", cursor::Show);
        self.out.flush();
    }
}
