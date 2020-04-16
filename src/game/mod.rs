use termion::cursor;
use termion::color::*;
use termion::raw::*;
use std::io::{self, Write};

pub mod tetromino;
use tetromino::*;

const HEIGHT: usize = 20;
const WIDTH:  usize = 10;

pub struct Game {
    grid: [[Option<u8>; WIDTH]; HEIGHT],
    score: u32,
    // pause: bool,
    tetromino: Tetromino,
    out: RawTerminal<io::Stdout>,
    pub over: bool,
}

impl Game {

    pub fn new() -> Self {
        let mut stdout = io::stdout().into_raw_mode().unwrap();
        write!(stdout, "{}{}", cursor::Hide, termion::clear::All);
        Game {
            grid: [[None; WIDTH]; HEIGHT],
            score: 0,
            // pause: false,
            over: false,
            out: stdout,
            tetromino: Tetromino::new_random(WIDTH)
        }
    }

    fn write_block(&mut self, i: usize, j: usize) {
        match self.grid[i][j] {
            Some(0) => write!(self.out, "{}  {}", Bg(Red),     Bg(Reset)),
            Some(1) => write!(self.out, "{}  {}", Bg(Green),   Bg(Reset)),
            Some(2) => write!(self.out, "{}  {}", Bg(Blue),    Bg(Reset)),
            Some(3) => write!(self.out, "{}  {}", Bg(Cyan),    Bg(Reset)),
            Some(4) => write!(self.out, "{}  {}", Bg(Magenta), Bg(Reset)),
            Some(5) => write!(self.out, "{}  {}", Bg(Yellow),  Bg(Reset)),
            Some(_) => unreachable!(),
            None    => write!(self.out, "  ")
        };
    }

    pub fn render(&mut self) {
        self.draw_piece(true);
        let wall = format!("{} {}", Bg(Black), Bg(Reset));
        write!(self.out, "{}Score: {}\r\n", cursor::Goto(1, 1), self.score);
        for i in 0..HEIGHT {
            write!(self.out, "{}", wall);
            (0..WIDTH).for_each(|j| self.write_block(i, j));
            write!(self.out, "{}\r\n", wall);
        }
        let bottom = (0..2*WIDTH+2).map(|_| "▀").collect::<String>();
        write!(self.out, "{}{}{}\r\n", Fg(Black), bottom, Fg(Reset));
        self.out.flush();
        self.draw_piece(false);
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
            if full {
                self.score += 1;
                for k in (1..i+1).rev() {
                    let prev_row = self.grid[k-1].clone();
                    self.grid[k] = prev_row;
                }
                self.grid[0] = [None; WIDTH];
            }
        }
    }

    pub fn shift(&mut self, dir: Direction) {
        let (left, right, down) = self.piece_touches();
        let touch = match dir {
            Direction::Left  => left,
            Direction::Right => right,
            Direction::Down  => down
        };
        if touch { return; }
        self.tetromino.shift(dir);
    }

    fn piece_touches(&self) -> (bool, bool, bool) {
        let cells = self.tetromino.cells;
        let left = cells.iter().any(|cell|
            cell.1 == 0 
            || self.grid[cell.0][cell.1-1].is_some()
        );
        let right = cells.iter().any(|cell|
            cell.1 == WIDTH-1 
            || self.grid[cell.0][cell.1+1].is_some()
        );
        let down = cells.iter().any(|cell|
            cell.0 == HEIGHT-1
            || self.grid[cell.0+1][cell.1].is_some()
        );
        (left, right, down)
    }

    pub fn turn(&mut self) {
        let backup = self.tetromino.cells.clone();
        if self.tetromino.turn().is_ok() { 
            let in_bounds = self.tetromino.cells.iter().all(|cell|
                cell.0 < HEIGHT && cell.1 < WIDTH
                && self.grid[cell.0][cell.1].is_none()
            );
            if in_bounds { return; }
        } 
        self.tetromino.cells = backup;
    }

    pub fn hard_drop(&mut self) {
        while !self.piece_touches().2 {
            self.shift(Direction::Down);
        }
    }

    pub fn tick(&mut self) {
        if !self.piece_touches().2 {
            self.shift(Direction::Down);
        } else {
            self.draw_piece(true);
            self.clear_lines();
            self.tetromino = Tetromino::new_random(WIDTH);
            self.over = self.tetromino.cells.iter().any(|cell|
                self.grid[cell.0][cell.1].is_some()
            );
        }
    }
}

impl Drop for Game {
    fn drop(&mut self) {
        write!(self.out, "{}", cursor::Show);
        self.out.flush();
    }
}

