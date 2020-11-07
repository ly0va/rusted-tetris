use rand::{rngs::SmallRng, Rng, SeedableRng};

const TETROMINOS: [[(usize, usize); 4]; 7] = [
    [(0, 0), (1, 0), (2, 0), (3, 0)],
    [(0, 0), (1, 0), (2, 0), (0, 1)],
    [(0, 0), (1, 0), (2, 0), (1, 1)],
    [(0, 0), (1, 0), (2, 0), (2, 1)],
    [(0, 0), (1, 0), (2, 1), (1, 1)],
    [(0, 1), (1, 0), (2, 0), (1, 1)],
    [(0, 0), (1, 0), (0, 1), (1, 1)],
];

pub enum Direction {
    Left,
    Right,
    Down,
}

pub struct Tetromino {
    pub cells: [(usize, usize); 4],
    pub color: u8,
}

impl Tetromino {
    pub fn new(index: usize, color: u8) -> Self {
        Tetromino {
            cells: TETROMINOS[index],
            color,
        }
    }

    pub fn new_random(width: usize) -> Self {
        let mut rng = SmallRng::from_entropy();
        let t = rng.gen_range(0, TETROMINOS.len());
        let c = rng.gen_range(0, 6);
        let mut tetromino = Self::new(t, c);
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

    pub fn turn(&mut self) -> Option<()> {
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
        let mut t = Tetromino::new(0, 0);
        t.shift(Direction::Down);
        assert_eq!(t.cells, [(1, 0), (2, 0), (3, 0), (4, 0)]);
        t.shift(Direction::Right);
        assert_eq!(t.cells, [(1, 1), (2, 1), (3, 1), (4, 1)]);
    }

    #[test]
    fn turn() {
        let mut t = Tetromino::new(2, 0); // T
        t.shift(Direction::Right);
        t.turn().unwrap();
        assert_eq!(t.cells, [(1, 2), (1, 1), (1, 0), (2, 1)]);
    }
}
