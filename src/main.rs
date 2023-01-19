use life_rs::game::{Grid, Cell};
use macroquad::prelude::*;

#[macroquad::main("life")]
async fn main() {
    let mut grid = Grid::new(100, 100);

    // glider: 
    grid.cells[0][2] = Cell::Alive;
    grid.cells[1][0] = Cell::Alive;
    grid.cells[1][2] = Cell::Alive;
    grid.cells[2][1] = Cell::Alive;
    grid.cells[2][2] = Cell::Alive;
    // the grid's origin is at the top-left corner of the screen.

    let tps = 10.0;             // ticks per second.
    let mut spent_time = 0.0;   // time spent since last tick.
    let mut zoom = 1.0 / 30.0;  // 30 cells enter in a screen width by def.

    loop {
        clear_background(BLACK);

        if spent_time >= 1. / tps {
            grid = grid.tick();
            spent_time = 0.0;
        }
        spent_time += get_frame_time();

        grid.render(zoom);
        zoom += (mouse_wheel().1 * get_frame_time()) / 30.;

        next_frame().await
    }
}
