use life_rs::data::{Grid, Cell};
use macroquad::prelude::*;

#[macroquad::main("life")]
async fn main() {
    let mut grid = Grid::new(10, 10);

    // blinker: 
    grid.cells[0][3] = Cell::Alive;
    grid.cells[1][3] = Cell::Alive;
    grid.cells[2][3] = Cell::Alive;

    let grid = grid.tick();

    assert_eq!(grid.cells[0][3], Cell::Dead);
    assert_eq!(grid.cells[2][3], Cell::Dead);
    assert_eq!(grid.cells[1][2], Cell::Alive);
    assert_eq!(grid.cells[1][4], Cell::Alive);

    //loop {
    //    clear_background(WHITE);
    //    next_frame().await
    //}
}
