use macroquad::prelude::*;

fn draw_ui(target_pos: Vec2, has_target: bool) -> bool {
    const OUTER: f32 = 10.0;
    let info_h = screen_height() * 0.15;
    let info_y = screen_height() - info_h - OUTER;

    let obs_size = vec2(screen_width() * 0.95, screen_height() * 0.6);
    let obs_pos = vec2(
        (screen_width() - obs_size.x) / 2.0,
        (info_y - obs_size.y) / 2.0,
    );

    draw_rectangle(obs_pos.x, obs_pos.y, obs_size.x, obs_size.y, DARKGRAY);
    draw_rectangle_lines(obs_pos.x, obs_pos.y, obs_size.x, obs_size.y, 4.0, WHITE);

    draw_rectangle(OUTER, info_y, screen_width() - OUTER * 2.0, info_h, DARKGRAY);
    draw_rectangle_lines(OUTER, info_y, screen_width() - OUTER * 2.0, info_h, 4.0, WHITE);

    let (mx, my) = mouse_position();
    let inside = mx > obs_pos.x
        && mx < obs_pos.x + obs_size.x
        && my > obs_pos.y
        && my < obs_pos.y + obs_size.y;

    let text = if inside {
        "Mouse INSIDE observation area"
    } else {
        "Mouse OUTSIDE observation area"
    };
    let color = if inside { GREEN } else { RED };

    draw_text(text, OUTER + 10.0, info_y + 30.0, 28.0, color);

    if has_target && inside {
        draw_circle(target_pos.x, target_pos.y, 8.0, GREEN);
        draw_circle_lines(target_pos.x, target_pos.y, 8.0, 3.0, WHITE);
    }

    inside
}

#[macroquad::main("i say move, agent moves and i say stop, agent stops. i happy now")]
async fn main() {
    let mut has_target = false;
    let mut target_pos = vec2(0.0, 0.0);

    loop {
        clear_background(BLACK);

        let inside_obs = draw_ui(target_pos, has_target);

        if is_mouse_button_pressed(MouseButton::Left) && inside_obs {
            target_pos = mouse_position().into();
            has_target = true;
            println!("Target set → ({:.1}, {:.1})", target_pos.x, target_pos.y);
        }

        if is_mouse_button_pressed(MouseButton::Right) {
            if has_target {
                println!("Target cleared");
                has_target = false;
            }
        }

        let status = if has_target { "TARGET LOCKED" } else { "NO TARGET" };
        let col = if has_target { GREEN } else { RED };
        draw_text(status, 20.0, screen_height() - 40.0, 40.0, col);

        next_frame().await
    }
}