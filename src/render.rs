use crate::game::{Grid, Cell};
use macroquad::prelude::*;

impl Grid {
    pub fn render(&self, zoom: f32) {
        let square_length = screen_width() * zoom;

        for (i, row) in self.cells.iter().enumerate() {
            for (j, cell) in row.iter().enumerate() {
                if *cell == Cell::Alive {
                    draw_rectangle(
                        i as f32 * square_length,
                        j as f32 * square_length,
                        square_length,
                        square_length, 
                        WHITE)     
                }
            }
        }
    }
}
