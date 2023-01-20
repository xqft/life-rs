#[repr(u8)]
#[derive(Copy, Clone, PartialEq)]
pub enum Cell {
    Dead = 0,
    Alive = 1
}

#[derive(Debug)]
#[derive(Clone, Copy)]
pub struct Position {
    pub row: usize,
    pub col: usize
}

pub struct Grid {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<Cell>,
}

pub struct Neighbourhood<'a> {
    grid: &'a Grid,
    current_idx: usize,
    center_idx: usize
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
            cells: vec![Cell::Dead; width * height],
        }
    }

    pub fn tick(&self) -> Self {
        let mut new = Grid::new(self.width, self.height);

        for (idx, cell) in self.cells.iter().enumerate() {
            new.cells[idx] = match (cell, self.get_alive_count(idx)) {
                (Cell::Alive, 0 | 1)    => Cell::Dead,
                (Cell::Alive, 4..)      => Cell::Dead,
                (Cell::Dead, 3)         => Cell::Alive,
                (other, _)              => *other, // stays the same
            };
        } 

        new
    }

    pub fn get_idx(&self, pos: Position) -> usize {
        pos.row * self.width + pos.col
    }

    pub fn get_pos(&self, idx: usize) -> Position {
        Position {
            row: idx / self.height,
            col: idx % self.width
        }
    }

    fn get_neighbourhood(&self, center_idx: usize) -> Neighbourhood {
        Neighbourhood { grid: &self, current_idx: 0, center_idx }
    }

    fn get_alive_count(&self, idx: usize) -> u8 {
        self.get_neighbourhood(idx).into_iter().sum()
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
                let center = self.grid.get_pos(self.center_idx);
                let row = (center.row + self.grid.height - 1 + self.current_idx / 3) % self.grid.height;
                let col = (center.col + self.grid.width  - 1 + self.current_idx % 3) % self.grid.width;
                //         ^^^^^^^^^^^^^^^   ^^^^^^^^^^^^^^^  ^^^   ^^^^^^^^^^^^^^^^^^^^  ^^^^^^^^^^^^^^^^^^^
                //        starts in center   avoids negatives  |     neighbour offset    wraps around the grid
                //                                    upper-left corners
                let idx = self.grid.get_idx(Position { row, col });

                self.current_idx += 1;

                Some(self.grid.cells[idx] as u8) 
            },
            _ => None
        } 
    }
}
