use crate::tetromino::*;

pub const WIDTH: usize = 10;
pub const HEIGHT: usize = 20;
pub type StandardGame = Game<WIDTH, HEIGHT>;

#[derive(Clone, Debug)]
pub struct Game<const WIDTH: usize, const HEIGHT: usize> {
    pub grid: [[Color; WIDTH]; HEIGHT],
    pub score: u32,
    pub tetromino: Tetromino,
    pub over: bool,
}

impl<const WIDTH: usize, const HEIGHT: usize> Game<WIDTH, HEIGHT> {
    pub fn new() -> Self {
        Game {
            grid: [[Color::None; WIDTH]; HEIGHT],
            score: 0,
            over: false,
            tetromino: Tetromino::new_random(WIDTH),
        }
    }

    pub fn draw_piece(&mut self, draw: bool) {
        for cell in &self.tetromino.cells {
            self.grid[cell.0][cell.1] = if draw {
                self.tetromino.color
            } else {
                Color::None
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
            for k in (1..=i).rev() {
                let prev_row = self.grid[k - 1];
                self.grid[k] = prev_row;
            }
            self.grid[0] = [Color::None; WIDTH];
        }
    }

    pub fn shift(&mut self, dir: Direction) {
        let (left, right, down) = self.piece_touches();
        let touch = match dir {
            Direction::Left => left,
            Direction::Right => right,
            Direction::Down => down,
        };
        if !touch {
            self.tetromino.shift(dir);
        }
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
            self.over = self
                .tetromino
                .cells
                .iter()
                .any(|cell| self.grid[cell.0][cell.1].is_some());
        }
    }
}
