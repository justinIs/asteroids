use macroquad::prelude::*;

mod asteroid;
mod ship;
mod vec_util;

fn window_conf() -> Conf {
    Conf {
        window_title: "Asteroids".to_owned(),
        window_width: 900,
        window_height: 900,
        window_resizable: false,
        ..Default::default()
    }
}

const ASTEROID_COUNT: usize = 5;

#[macroquad::main(window_conf)]
async fn main() {
    let mut paused = false;

    let ship_pos = vec2(screen_width() / 2.0, screen_height() / 2.0);
    let mut ship = ship::Ship::new(ship_pos);

    //                    ~ship_radius + buffer
    let asteroid_clearance = 20.0 + 5.0;

    let mut asteroids: Vec<asteroid::Asteroid> = Vec::with_capacity(ASTEROID_COUNT);
    for _ in 0..ASTEROID_COUNT {
        let pos = asteroid::Asteroid::gen_position(ship_pos, asteroid_clearance, &asteroids);
        asteroids.push(asteroid::Asteroid::new(asteroid::AsteroidSize::Large, pos));
    }

    let mut hit = false;

    loop {
        clear_background(BLACK);

        let dt = get_frame_time();

        if !paused {
            ship.update(dt);
            for a in &mut asteroids {
                a.update(dt);
                a.draw();
            }
            hit = asteroids.iter().any(|a| {
                let (ship_pos, ship_rad) = ship.bounds();
                let (a_pos, a_rad) = a.bounds();
                circles_overlap(ship_pos, ship_rad, a_pos, a_rad)
            });
        }

        if hit {
            paused = true;
        }

        ship.draw();
        for a in &asteroids {
            a.draw();
        }

        if hit && paused {
            draw_centered_outlined("HIT", 80.0, WHITE, BLACK);
        }

        next_frame().await
    }
}

fn circles_overlap(c1: Vec2, r1: f32, c2: Vec2, r2: f32) -> bool {
    c1.distance_squared(c2) < (r1 + r2).powi(2)
}

fn draw_centered_outlined(text: &str, font_size: f32, fill: Color, outline: Color) {
    let dims = measure_text(text, None, font_size as u16, 1.0);
    let x = (screen_width() - dims.width) / 2.0;
    let y = (screen_height() - dims.height) / 2.0;

    let t = 2.0;
    for (dx, dy) in [
        (-t, -t),
        (t, -t),
        (-t, t),
        (t, t),
        (-t, 0.0),
        (t, 0.0),
        (0.0, -t),
        (0.0, t),
    ] {
        draw_text(text, x + dx, y + dy, font_size, outline);
    }
    draw_text(text, x, y, font_size, fill);
}
