#![allow(dead_code)]

use console::*;
use std::thread;
use std::time::Duration;
use rand::prelude::*;

const HEIGHT: usize = 20;
const WIDTH:  usize = 10;

const TETROMINOS: [[(usize, usize); 4]; 7] = [
    [(0, 0), (1, 0), (2, 0), (3, 0)], 
    [(0, 0), (1, 0), (2, 0), (0, 1)], 
    [(0, 0), (1, 0), (2, 0), (1, 1)], 
    [(0, 0), (1, 0), (2, 0), (2, 1)], 
    [(0, 0), (1, 0), (2, 1), (1, 1)], 
    [(0, 1), (1, 0), (2, 0), (1, 1)],
    [(0, 0), (1, 0), (0, 1), (1, 1)]
];

const COLORS: [Color; 6] = [
    Color::Cyan,
    Color::Red,
    Color::Blue,
    Color::Green,
    Color::Yellow,
    Color::Magenta
];

struct Game {
    grid: [[Option<Color>; WIDTH]; HEIGHT],
    score: u32,
    game_over: bool,
    pause: bool,
    term: Term,
    tetromino: Tetromino
}

struct Tetromino {
    cells: [(usize, usize); 4],
    color: Color
}

impl Tetromino {

    fn new(index: usize, color: usize) -> Self {
        Tetromino {
            cells: TETROMINOS[index],
            color: COLORS[color]
        }
    }

    fn shift(&mut self, iinc: i32, jinc: i32) {
        for cell in self.cells.iter_mut() {
            cell.0 = (cell.0 as i32 + iinc) as usize;
            cell.1 = (cell.1 as i32 + jinc) as usize;
        }
    }

    fn turn(&mut self) {
        let center = self.cells[1];
        for &i in &[0, 2, 3] {
            self.cells[i].0 -= center.0;
            self.cells[i].1 -= center.1;
            self.cells[i] = (
                center.0 + self.cells[i].1, 
                center.1 - self.cells[i].0
            );
        }
    }
}

impl Game {

    fn new() -> Self {
        let term = Term::stdout();
        term.hide_cursor();
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

    fn render(&self) {
        self.term.clear_screen();
        for i in 0..HEIGHT {
            for j in 0..WIDTH {
                if let Some(color) = self.grid[i][j] {
                    let color = Style::new().bg(color);
                    print!("{}", color.apply_to("  "));
                } else {
                    print!("  ");
                }
            }
            println!();
        }
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
                    self.grid[k][..].copy_from_slice(&prev_row);
                }
                self.grid[0][..].copy_from_slice(&[None; WIDTH]);
            }
        }
    }
}

impl Drop for Game {
    fn drop(&mut self) {
        self.term.show_cursor();
    }
}

fn main() {
    let mut game = Game::new();
    for i in 0..15 {
        game.draw_piece(false);
        game.tetromino.shift(1, 0);
        game.draw_piece(true);
        game.render();
        thread::sleep(Duration::from_millis(500));
    }
}
