use crate::game::{Grid, Cell};
use macroquad::prelude::*;

impl Grid {
    pub fn render(&self, zoom: f32) {
        let square_length = screen_width() * zoom;

        for (idx, cell) in self.cells.iter().enumerate() {
            let pos = self.get_pos(idx);
            if *cell == Cell::Alive {
                draw_rectangle(
                    pos.row as f32 * square_length,
                    pos.col as f32 * square_length,
                    square_length,
                    square_length, 
                    WHITE)     
            }
        }
    }
}
