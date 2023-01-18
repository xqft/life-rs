use std::cmp;

#[derive(Copy, Clone, PartialEq)]
pub enum Cell {
    Dead,
    Alive
}

pub struct Grid {
    width: usize,
    height: usize,
    pub cells: Vec<Vec<Cell>>
}

impl Grid {
    pub fn new(width: usize, height: usize) -> Grid {
        Grid {
            width,
            height,
            cells: vec![vec![Cell::Dead; width]; height]
        }
    }

    pub fn tick(self) -> Grid {
        let mut new = Grid::new(self.width, self.height);

        for (i, row) in self.cells.iter().enumerate() {
            for (j, cell) in row.iter().enumerate() {
                match cell {
                    Cell::Dead => if self.live_neighbours(i, j) == 3 {
                        new.cells[i][j] = Cell::Alive
                    },
                    Cell::Alive => match self.live_neighbours(i, j) {
                        0 | 1 => new.cells[i][j] = Cell::Dead,
                        3.. => new.cells[i][j] = Cell::Dead,
                        _ => () // cell lives
                    }
                }
            }
        } 

        new
    }

    fn get_neighbours(&self, row: usize, col: usize) -> Vec<Cell> {
        let mut result = Vec::new();

        let imin = row.checked_sub(1).unwrap_or(0);
        let imax = cmp::min(self.height - 1, row + 1);
        let jmin = col.checked_sub(1).unwrap_or(0);
        let jmax = cmp::min(self.width - 1, col + 1);

        for i in imin..=imax {
            for j in jmin..=jmax {
                result.push(self.cells[i][j])
            }
        }

        result
    }

    fn live_neighbours(&self, row: usize, col: usize) -> usize {
        self.get_neighbours(row, col)
            .iter()
            .filter(|cell| **cell == Cell::Alive)
            .count()
    }
}
