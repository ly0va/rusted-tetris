use crate::tetromino::*;
use std::io::{self, Write};
use termion::{color::*, cursor, raw::*};

const HEIGHT: usize = 20;
const WIDTH: usize = 10;

pub struct Game {
    grid: [[Option<u8>; WIDTH]; HEIGHT],
    score: u32,
    pause: bool,
    tetromino: Tetromino,
    out: RawTerminal<io::Stdout>,
    pub over: bool,
}

impl Game {
    pub fn new() -> io::Result<Self> {
        let mut stdout = io::stdout().into_raw_mode()?;
        write!(stdout, "{}{}", cursor::Hide, termion::clear::All)?;
        Ok(Game {
            grid: [[None; WIDTH]; HEIGHT],
            score: 0,
            pause: false,
            over: false,
            out: stdout,
            tetromino: Tetromino::new_random(WIDTH),
        })
    }

    fn write_block(&mut self, i: usize, j: usize) -> io::Result<()> {
        macro_rules! color {
            ($c:expr) => {
                write!(self.out, "{}  {}", Bg($c), Bg(Reset))
            };
        }

        match self.grid[i][j] {
            Some(0) => color!(Red),
            Some(1) => color!(Green),
            Some(2) => color!(Blue),
            Some(3) => color!(Cyan),
            Some(4) => color!(Yellow),
            Some(5) => color!(Magenta),
            Some(_) => unreachable!(),
            None => write!(self.out, "  "),
        }
    }

    pub fn render(&mut self) -> io::Result<()> {
        self.draw_piece(true);
        write!(self.out, "{}", cursor::Goto(1, 1))?;
        let wall = format!("{} {}", Bg(White), Bg(Reset));
        for i in 0..HEIGHT {
            write!(self.out, "{}", wall)?;
            (0..WIDTH).try_for_each(|j| self.write_block(i, j))?;
            writeln!(self.out, "{}\r", wall)?;
        }
        let bottom = (0..2 * WIDTH + 2).map(|_| " ").collect::<String>();
        write!(self.out, "{}{}{}", Bg(White), Fg(Black), bottom)?;
        writeln!(
            self.out,
            "{} Score: {}{}{}\r",
            cursor::Goto(1, 1 + HEIGHT as u16),
            self.score,
            Bg(Reset),
            Fg(Reset)
        )?;
        self.out.flush()?;
        self.draw_piece(false);
        Ok(())
    }

    fn draw_piece(&mut self, draw: bool) {
        for cell in &self.tetromino.cells {
            self.grid[cell.0][cell.1] = if draw {
                Some(self.tetromino.color)
            } else {
                None
            }
        }
    }

    fn clear_lines(&mut self) {
        for i in 0..HEIGHT {
            let full = self.grid[i].iter().all(|x| x.is_some());
            if !full {
                continue;
            }
            self.score += 1;
            for k in (1..i + 1).rev() {
                let prev_row = self.grid[k - 1];
                self.grid[k] = prev_row;
            }
            self.grid[0] = [None; WIDTH];
        }
    }

    pub fn shift(&mut self, dir: Direction) {
        if self.pause {
            return;
        }
        let (left, right, down) = self.piece_touches();
        let touch = match dir {
            Direction::Left => left,
            Direction::Right => right,
            Direction::Down => down,
        };
        if touch {
            return;
        }
        self.tetromino.shift(dir);
    }

    fn piece_touches(&self) -> (bool, bool, bool) {
        let cells = self.tetromino.cells;
        let left = cells
            .iter()
            .any(|cell| cell.1 == 0 || self.grid[cell.0][cell.1 - 1].is_some());
        let right = cells
            .iter()
            .any(|cell| cell.1 == WIDTH - 1 || self.grid[cell.0][cell.1 + 1].is_some());
        let down = cells
            .iter()
            .any(|cell| cell.0 == HEIGHT - 1 || self.grid[cell.0 + 1][cell.1].is_some());
        (left, right, down)
    }

    pub fn turn(&mut self) {
        if self.pause {
            return;
        }
        let backup = self.tetromino.cells;
        if self.tetromino.turn().is_some() {
            let in_bounds = self.tetromino.cells.iter().all(|cell| {
                cell.0 < HEIGHT && cell.1 < WIDTH && self.grid[cell.0][cell.1].is_none()
            });
            if in_bounds {
                return;
            }
        }
        self.tetromino.cells = backup;
    }

    pub fn hard_drop(&mut self) {
        if self.pause {
            return;
        }
        while !self.piece_touches().2 {
            self.shift(Direction::Down);
        }
    }

    pub fn tick(&mut self) {
        if self.pause {
            return;
        }
        if !self.piece_touches().2 {
            self.shift(Direction::Down);
        } else {
            self.draw_piece(true);
            self.clear_lines();
            self.tetromino = Tetromino::new_random(WIDTH);
            self.over = self
                .tetromino
                .cells
                .iter()
                .any(|cell| self.grid[cell.0][cell.1].is_some());
        }
    }

    pub fn toggle_pause(&mut self) {
        self.pause = !self.pause;
    }
}

impl Drop for Game {
    #[allow(unused_must_use)]
    fn drop(&mut self) {
        write!(self.out, "{}", cursor::Show);
        self.out.flush();
    }
}
