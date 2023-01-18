use std::cmp;

#[derive(Debug)]
#[derive(Copy, Clone, PartialEq)]
pub enum Cell {
    Dead,
    Alive
}

#[derive(Debug)]
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
                new.cells[i][j] = match (cell, self.live_neighbours(i, j)) {
                    (Cell::Alive, 0 | 1)    => Cell::Dead,
                    (Cell::Alive, 4..)      => Cell::Dead,
                    (Cell::Dead, 3)         => Cell::Alive,
                    (other, _)              => *other, // stays the same
                };
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

    pub fn live_neighbours(&self, row: usize, col: usize) -> usize {
        self.get_neighbours(row, col)
            .iter()
            .filter(|cell| **cell == Cell::Alive)
            .count()
            - if self.cells[row][col] == Cell::Alive { 1 } else { 0 } // so it doesn't count itself.
    }
}
