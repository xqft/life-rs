use macroquad::prelude::*;

#[macroquad::main("conway")]
async fn main() {

    loop {
        clear_background(WHITE);
        next_frame().await
    }
}
