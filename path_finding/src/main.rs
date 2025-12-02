use macroquad::prelude::*;
fn simulation_area() {
    let area_width = screen_width() * 0.95;
    let area_height = screen_height() * 0.8;
    let area_x = (screen_width() - area_width) / 2.0;
    let area_y = (screen_height() - area_height) / 2.0;

    draw_rectangle(area_x, area_y, area_width, area_height, DARKGRAY);
    draw_rectangle_lines(area_x, area_y, area_width, area_height, 2.0, WHITE);
}

#[macroquad::main("pathfinding")]
async fn main() {
    loop {
        clear_background(BLACK);
        simulation_area();
        next_frame().await;
    }
}
