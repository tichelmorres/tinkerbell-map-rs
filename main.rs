use raylib::prelude::*;
use std::time::{Duration, Instant};

const WINDOW_WIDTH		 : i32	 = 1280;
const WINDOW_HEIGHT		 : i32	 = 720;

const BACKGROUND_COLOR	 : Color = Color::new(194, 189, 166, 255);
const FOREGROUND_COLOR	 : Color = Color::new(157, 0, 6, 255);

const SQUARE_BASE_SIZE	 : f32	 = 5.0;
const INITIAL_ZOOM_LEVEL : f64	 = 320.0;
const MIN_ZOOM			 : f64	 = 0.56;
const MAX_ZOOM			 : f64	 = 5600.0;

const TOTAL_STEPS		 : usize = 10000;
const STEP_INTERVAL_MC	 : u64	 = 2000;

const X_OFFSET			 : f32	 = 80.0;
const Y_OFFSET			 : f32	 = 160.0;

const INTERVAL_LABEL_X	 : i32	 = 240;

fn calculate_next_position(x: f32, y: f32) -> Vector2 {
    let a = 0.9_f32;
    let b = -0.6013_f32;
    let c = 2.0_f32;
    let d = 0.5_f32;

    Vector2::new(
        x * x - y * y + a * x + b * y,
        2.0 * x * y + c * x + d * y,
    )
}

fn map_to_screen(v: Vector2, zoom_level: f64) -> Vector2 {
    Vector2::new(
        (WINDOW_WIDTH as f32)  * 0.5 + v.x * (zoom_level as f32) + X_OFFSET,
        (WINDOW_HEIGHT as f32) * 0.5 - v.y * (zoom_level as f32) - Y_OFFSET,
    )
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .title("Tinkerbell Map")
        .build();

    rl.set_target_fps(144);

    let mut zoom_level = INITIAL_ZOOM_LEVEL;

    let mut current_point = Vector2::new(-0.73, -0.64);
    let mut trail: Vec<Vector2> = Vec::new();
    trail.push(current_point);

    let mut steps_taken: usize = 0;
    let interval = Duration::from_micros(STEP_INTERVAL_MC);
    let mut last_update = Instant::now();
    let mut paused = true;

    while !rl.window_should_close() {
        let wheel_step_normal = 0.12;
        let wheel_step_fine = 0.03;
        let mouse_wheel_delta = rl.get_mouse_wheel_move();
        if mouse_wheel_delta != 0.0 {
            let ctrl_held = rl.is_key_down(KeyboardKey::KEY_LEFT_CONTROL)
                || rl.is_key_down(KeyboardKey::KEY_RIGHT_CONTROL);
            let step = if ctrl_held { wheel_step_fine } else { wheel_step_normal };
            let zoom_multiplier = (1.0 + (mouse_wheel_delta as f64) * step).max(1e-12);
            zoom_level *= zoom_multiplier;
            zoom_level = zoom_level.clamp(MIN_ZOOM, MAX_ZOOM);
        }

        let arrow_step_normal = 0.03;
        let arrow_step_fine = 0.01;
        let arrow_delta = match (
            rl.is_key_down(KeyboardKey::KEY_UP),
            rl.is_key_down(KeyboardKey::KEY_DOWN),
        ) {
            (true, false) => 1.0,
            (false, true) => -1.0,
            _ => 0.0,
        };
        if arrow_delta != 0.0 {
            let ctrl_held = rl.is_key_down(KeyboardKey::KEY_LEFT_CONTROL)
                || rl.is_key_down(KeyboardKey::KEY_RIGHT_CONTROL);
            let step = if ctrl_held { arrow_step_fine / 5.0 } else { arrow_step_normal / 5.0 };
            let zoom_multiplier = (1.0 + (arrow_delta as f64) * step).max(1e-12);
            zoom_level *= zoom_multiplier;
            zoom_level = zoom_level.clamp(MIN_ZOOM, MAX_ZOOM);
        }

        if rl.is_key_pressed(KeyboardKey::KEY_P) {
            paused = !paused;
            if !paused {
                last_update = Instant::now();
            }
        }

        if !paused {
            let now = Instant::now();
            while steps_taken < TOTAL_STEPS && now.duration_since(last_update) >= interval {
                current_point = calculate_next_position(current_point.x, current_point.y);
                trail.push(current_point);
                steps_taken += 1;
                last_update += interval;
            }
        }

        if rl.is_key_pressed(KeyboardKey::KEY_Q) {
            break;
        }

        if rl.is_key_pressed(KeyboardKey::KEY_R) {
            zoom_level = INITIAL_ZOOM_LEVEL;
            current_point = Vector2::new(-0.73, -0.64);
            trail.clear();
            trail.push(current_point);
            steps_taken = 0;
            last_update = Instant::now();
        }

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(BACKGROUND_COLOR);

        let pixel_size_vec = Vector2::new(SQUARE_BASE_SIZE, SQUARE_BASE_SIZE);
        for math_point in &trail {
            let screen_point = map_to_screen(*math_point, zoom_level);
            d.draw_rectangle_v(screen_point, pixel_size_vec, FOREGROUND_COLOR);
        }

        let steps_text = format!("Steps: {}/{}", steps_taken, TOTAL_STEPS);
        d.draw_text(&steps_text, 10, 10, 20, FOREGROUND_COLOR);

        let interval_text = format!("({} micros / step)", STEP_INTERVAL_MC);
        d.draw_text(&interval_text, INTERVAL_LABEL_X, 10, 20, FOREGROUND_COLOR);

        d.draw_text(&format!("Zoom: {:.8}", zoom_level), 10, 32, 20, FOREGROUND_COLOR);
        d.draw_text("Press R to reset.", 10, 54, 20, FOREGROUND_COLOR);
        d.draw_text("Press P to toggle pause.", 10, 76, 20, FOREGROUND_COLOR);

        if steps_taken == TOTAL_STEPS {
            d.draw_text("Done.", 10, 690, 20, FOREGROUND_COLOR);
        } else if paused {
            d.draw_text("Paused.", 10, 690, 20, FOREGROUND_COLOR);
        } else {
            d.draw_text("Computing...", 10, 690, 20, FOREGROUND_COLOR);
        }
    }
}
