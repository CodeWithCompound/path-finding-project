use macroquad::prelude::*;

fn draw_ui(agent_pos: Vec2, target_pos: Vec2, has_target: bool) -> bool {
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
    draw_rectangle(
        OUTER,
        info_y,
        screen_width() - OUTER * 2.0,
        info_h,
        DARKGRAY,
    );
    draw_rectangle_lines(
        OUTER,
        info_y,
        screen_width() - OUTER * 2.0,
        info_h,
        4.0,
        WHITE,
    );

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

    draw_circle(agent_pos.x, agent_pos.y, 10.0, BLUE);
    draw_circle_lines(agent_pos.x, agent_pos.y, 10.0, 3.0, WHITE);

    if has_target {
        draw_circle(target_pos.x, target_pos.y, 8.0, GREEN);
        draw_circle_lines(target_pos.x, target_pos.y, 8.0, 3.0, WHITE);

let direction = (target_pos - agent_pos).normalize_or_zero();
    let arrow_tip = agent_pos + direction * 18.0;
    let perpendicular = vec2(-direction.y, direction.x);

    draw_triangle(
        arrow_tip,
        agent_pos + direction * 8.0 + perpendicular * 7.0,
        agent_pos + direction * 8.0 - perpendicular * 7.0,
        YELLOW,
    );
}

    inside
}

#[macroquad::main("i say move, agent moves and i say stop, agent stops. i happy now")]
async fn main() {
    let mut has_target = false;
    let mut target_pos = vec2(0.0, 0.0);
    let mut agent_pos = vec2(200.0, 200.0);
    const MOVE_SPEED: f32 = 180.0;

    loop {
        clear_background(BLACK);

        let inside_obs = draw_ui(agent_pos, target_pos, has_target);

        if has_target && inside_obs {
            let dir = (target_pos - agent_pos).normalize_or_zero();
            let move_this_frame = dir * MOVE_SPEED * get_frame_time();

            if agent_pos.distance(target_pos) > 12.0 {
                agent_pos += move_this_frame;
            } else {
                agent_pos = target_pos;
            }
        }

        if is_mouse_button_pressed(MouseButton::Left) && inside_obs {
            target_pos = mouse_position().into();
            has_target = true;
            println!("Target set to ({:.1}, {:.1})", target_pos.x, target_pos.y);
        }

        if is_mouse_button_pressed(MouseButton::Right) {
            has_target = false;
            println!("Target cleared");
        }

        let status = if has_target {
            "TARGET LOCKED"
        } else {
            "NO TARGET"
        };
        let col = if has_target { GREEN } else { RED };
        draw_text(status, 20.0, screen_height() - 40.0, 40.0, col);

        next_frame().await
    }
}