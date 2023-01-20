use life_rs::game::{Grid, Cell, Position};
use macroquad::prelude::*;

#[macroquad::main("life")]
async fn main() {
    let mut grid = Grid::new(100, 100);

    // glider: 
    let positions = [
        Position { row: 0, col: 2},
        Position { row: 1, col: 0},
        Position { row: 1, col: 2},
        Position { row: 2, col: 1},
        Position { row: 2, col: 2}
    ];

    for pos in positions {
        let idx = grid.get_idx(pos);
        grid.cells[idx] = Cell::Alive;
    }
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
