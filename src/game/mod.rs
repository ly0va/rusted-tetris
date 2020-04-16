use console::*;
use rand::prelude::*;

pub mod tetromino;
use tetromino::*;

const HEIGHT: usize = 20;
const WIDTH:  usize = 10;

pub struct Game {
    grid: [[Option<Color>; WIDTH]; HEIGHT],
    score: u32,
    game_over: bool,
    pause: bool,
    tetromino: Tetromino,
    pub term: Term
}

impl Game {

    pub fn new() -> Self {
        let term = Term::buffered_stdout();
        term.hide_cursor();
        term.clear_screen();
        let mut rng = rand::thread_rng();
        let t = rng.gen_range(0, TETROMINOS.len());
        let c = rng.gen_range(0, COLORS.len());
        Game {
            grid: [[None; WIDTH]; HEIGHT],
            score: 0,
            pause: false,
            game_over: false,
            term: term,
            tetromino: Tetromino::new(t, c)
        }
    }

    pub fn render(&mut self) {
        self.term.clear_last_lines(HEIGHT);
        self.draw_piece(true);
        for i in 0..HEIGHT {
            for j in 0..WIDTH {
                match self.grid[i][j] {
                    Some(color) => {
                        let color = Style::new().bg(color);
                        self.term.write_str(&format!("{}", color.apply_to("  ")));
                    }
                    None => { self.term.write_str("  "); }
                }
            }
            self.term.write_str("\r\n");
        }
        self.term.flush();
        self.draw_piece(false);
    }

    pub fn draw_piece(&mut self, draw: bool) {
        for cell in &self.tetromino.cells {
            self.grid[cell.0][cell.1] = if draw { 
                Some(self.tetromino.color) 
            } else {
                None 
            }
        }
    }

    pub fn clear_lines(&mut self) {
        for i in 0..HEIGHT {
            let full = self.grid[i].iter().all(|x| x.is_some());
            if full {
                self.score += 1;
                for k in (1..i+1).rev() {
                    let prev_row = self.grid[k-1].clone();
                    self.grid[k][..].copy_from_slice(&prev_row);
                }
                self.grid[0][..].copy_from_slice(&[None; WIDTH]);
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
}

impl Drop for Game {
    fn drop(&mut self) {
        self.term.show_cursor();
        self.term.flush();
    }
}

