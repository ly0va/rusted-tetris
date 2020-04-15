use console::Color;

pub const TETROMINOS: [[(usize, usize); 4]; 7] = [
    [(0, 0), (1, 0), (2, 0), (3, 0)], 
    [(0, 0), (1, 0), (2, 0), (0, 1)], 
    [(0, 0), (1, 0), (2, 0), (1, 1)], 
    [(0, 0), (1, 0), (2, 0), (2, 1)], 
    [(0, 0), (1, 0), (2, 1), (1, 1)], 
    [(0, 1), (1, 0), (2, 0), (1, 1)],
    [(0, 0), (1, 0), (0, 1), (1, 1)]
];

pub const COLORS: [Color; 6] = [
    Color::Cyan,
    Color::Red,
    Color::Blue,
    Color::Green,
    Color::Yellow,
    Color::Magenta
];

pub struct Tetromino {
    pub cells: [(usize, usize); 4],
    pub color: Color
}

impl Tetromino {

    pub fn new(index: usize, color: usize) -> Self {
        Tetromino {
            cells: TETROMINOS[index],
            color: COLORS[color]
        }
    }

    pub fn shift(&mut self, iinc: i32, jinc: i32) {
        for cell in self.cells.iter_mut() {
            cell.0 = (cell.0 as i32 + iinc) as usize;
            cell.1 = (cell.1 as i32 + jinc) as usize;
        }
    }

    pub fn turn(&mut self) {
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

