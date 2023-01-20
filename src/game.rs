#[repr(u8)]
#[derive(Copy, Clone, PartialEq)]
pub enum Cell {
    Dead = 0,
    Alive = 1
}

pub struct Position {
    row: usize,
    col: usize
}

pub struct Grid {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<Vec<Cell>>
}

pub struct Neighbourhood<'a> {
    grid: &'a Grid,
    current_idx: usize,
    center: Position
}
// every neighbour has its own index, from 0 to 8.
// [0][1][2]
// [3][4][5]
// [6][7][8]
// 4th index corresponds to the center, so it needs to be skiped.

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
                new.cells[i][j] = match (cell, self.get_alive_count(Position {row: i, col: j})) {
                    (Cell::Alive, 0 | 1)    => Cell::Dead,
                    (Cell::Alive, 4..)      => Cell::Dead,
                    (Cell::Dead, 3)         => Cell::Alive,
                    (other, _)              => *other, // stays the same
                };
            }
        } 

        new
    }

    fn get_neighbourhood(&self, center: Position) -> Neighbourhood {
        Neighbourhood { grid: &self, current_idx: 0, center }
    }

    fn get_alive_count(&self, center: Position) -> u8 {
        self.get_neighbourhood(center).into_iter().sum()
    }
}

impl<'a> Iterator for Neighbourhood<'a> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
       match self.current_idx {
            4 => { // 4th index corresponds to the center, which isn't a neighbour.
                self.current_idx += 1;
                self.next()
            },
            0..=8 => {
                let row = (self.center.row + self.grid.height - 1 + self.current_idx / 3) % self.grid.height;
                let col = (self.center.col + self.grid.width  - 1 + self.current_idx % 3) % self.grid.width;
                //         ^^^^^^^^^^^^^^^   ^^^^^^^^^^^^^^^  ^^^   ^^^^^^^^^^^^^^^^^^^^  ^^^^^^^^^^^^^^^^^^^
                //        starts in center   avoids negatives  |     neighbour offset    wraps around the grid
                //                                    upper-left corners

                self.current_idx += 1;

                Some(self.grid.cells[row][col] as u8) 
            },
            _ => None
        } 
    }
}
