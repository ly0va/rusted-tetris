use console::*;
use rand::prelude::*;

mod tetromino;
use tetromino::*;

const HEIGHT: usize = 20;
const WIDTH:  usize = 10;

pub struct Game {
    grid: [[Option<Color>; WIDTH]; HEIGHT],
    score: u32,
    game_over: bool,
    pause: bool,
    term: Term,
    tetromino: Tetromino
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

    pub fn render(&self) {
        self.term.clear_last_lines(HEIGHT);
        for i in 0..HEIGHT {
            let mut line = String::new();
            for j in 0..WIDTH {
                match self.grid[i][j] {
                    Some(color) => {
                        let color = Style::new().bg(color);
                        line += &format!("{}", color.apply_to("  "));
                    }
                    None => line += "  "
                }
            }
            line += "\r";
            self.term.write_line(&line);
        }
        self.term.flush();
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

    pub fn shift(&mut self, iinc: i32, jinc: i32) {
        self.draw_piece(false);
        self.tetromino.shift(iinc, jinc);
        self.draw_piece(true);
    }
}

impl Drop for Game {
    fn drop(&mut self) {
        self.term.show_cursor();
        self.term.flush();
    }
}

