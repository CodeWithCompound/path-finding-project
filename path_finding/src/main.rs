use std::vec;

use macroquad::prelude::*;
fn main_ui_elements() {
    let outer_offset = 10.0;
        let info_height = screen_height() * 0.15;
    let info_y = screen_height() - info_height - outer_offset;
    // from here on out o = observation
    let o_ui_dim = vec2(
        screen_width() * 0.95, 
        screen_height() * 0.6);
    let o_ui_pos = vec2(
        (screen_width() - o_ui_dim.x) / 2.0,
        (info_y - o_ui_dim.y) / 2.0,
    );

    draw_rectangle(o_ui_pos.x, o_ui_pos.y, o_ui_dim.x, o_ui_dim.y, DARKGRAY);
    draw_rectangle_lines(
        o_ui_pos.x, o_ui_pos.y, o_ui_dim.x, o_ui_dim.y, 2.0, DARKGRAY,
    );

    draw_rectangle(outer_offset, info_y, screen_width() - outer_offset * 2.0, info_height, DARKGRAY);
    draw_rectangle_lines(outer_offset, info_y, screen_width() - outer_offset * 2.0, info_height, 2.0, WHITE);
}
fn new_position(agent_has_target: &mut bool) {
    let (mouse_x, mouse_y) = mouse_position();
    println!("New position at x: {}, y: {}", mouse_x, mouse_y);
    *agent_has_target = true;
println!("Agent has target: {}", agent_has_target);
}
fn delete_position(agent_has_target: &mut bool) {
    if *agent_has_target {
        println!("Deleting target position.");
        *agent_has_target = false;
    } else {
        println!("No target position to delete.");
    }
    println!("Agent has target: {}", agent_has_target);
}

#[macroquad::main("pathfinding")]
async fn main() {
    let mut agent_has_target = false;
    loop {
        clear_background(BLACK);
        main_ui_elements();
        if is_mouse_button_pressed(MouseButton::Left) {
    new_position(&mut agent_has_target);
}
        if is_mouse_button_pressed(MouseButton::Right) {
    delete_position(&mut agent_has_target);
}

        next_frame().await;

    }
}
