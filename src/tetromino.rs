use rand::{prelude::SliceRandom, Rng};
use std::fmt;
use termion::color::{self, Color as TermionColor};

#[derive(Clone, Copy, Debug)]
pub enum Color {
    None,
    Red,
    Green,
    Blue,
    Magenta,
    Yellow,
    Cyan,
}

impl Color {
    const ALL: [Color; 6] = [
        Self::Red,
        Self::Green,
        Self::Blue,
        Self::Magenta,
        Self::Yellow,
        Self::Cyan,
    ];

    pub fn is_none(&self) -> bool {
        matches!(self, Self::None)
    }

    pub fn is_some(&self) -> bool {
        !matches!(self, Self::None)
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Red => color::Red.write_bg(f),
            Self::Green => color::Green.write_bg(f),
            Self::Blue => color::Blue.write_bg(f),
            Self::Magenta => color::Magenta.write_bg(f),
            Self::Yellow => color::Yellow.write_bg(f),
            Self::Cyan => color::Cyan.write_bg(f),
            Self::None => Ok(()),
        }
    }
}

pub enum Direction {
    Left,
    Right,
    Down,
}

const TETROMINOS: [[(usize, usize); 4]; 7] = [
    [(0, 0), (1, 0), (2, 0), (3, 0)],
    [(0, 0), (1, 0), (2, 0), (0, 1)],
    [(0, 0), (1, 0), (2, 0), (1, 1)],
    [(0, 0), (1, 0), (2, 0), (2, 1)],
    [(0, 0), (1, 0), (2, 1), (1, 1)],
    [(0, 1), (1, 0), (2, 0), (1, 1)],
    [(0, 0), (1, 0), (0, 1), (1, 1)],
];

#[derive(Clone, Debug)]
pub struct Tetromino {
    pub cells: [(usize, usize); 4],
    pub color: Color,
}

impl Tetromino {
    pub fn new(index: usize, color: Color) -> Self {
        Tetromino {
            cells: TETROMINOS[index],
            color,
        }
    }

    pub fn new_with_rng(width: usize, rng: &mut impl Rng) -> Self {
        let t = rng.gen_range(0, TETROMINOS.len());
        let c = Color::ALL.choose(rng).unwrap();
        let mut tetromino = Self::new(t, *c);
        let center = rng.gen_range(0, width - 2);
        for cell in tetromino.cells.iter_mut() {
            cell.1 += center;
        }
        tetromino
    }

    pub fn shift(&mut self, dir: Direction) {
        for cell in self.cells.iter_mut() {
            match dir {
                Direction::Left => cell.1 -= 1,
                Direction::Right => cell.1 += 1,
                Direction::Down => cell.0 += 1,
            }
        }
    }

    pub fn rotate(&mut self) -> Option<()> {
        let (center_y, center_x) = self.cells[1];
        for &i in &[0, 2, 3] {
            let (y, x) = self.cells[i];
            self.cells[i] = (
                (center_y + x).checked_sub(center_x)?,
                (center_x + center_y).checked_sub(y)?,
            );
        }
        Some(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shift() {
        let mut t = Tetromino::new(0, Color::None);
        t.shift(Direction::Down);
        assert_eq!(t.cells, [(1, 0), (2, 0), (3, 0), (4, 0)]);
        t.shift(Direction::Right);
        assert_eq!(t.cells, [(1, 1), (2, 1), (3, 1), (4, 1)]);
    }

    #[test]
    fn turn() {
        let mut t = Tetromino::new(2, Color::None); // T
        t.shift(Direction::Right);
        t.rotate().unwrap();
        assert_eq!(t.cells, [(1, 2), (1, 1), (1, 0), (2, 1)]);
    }
}
