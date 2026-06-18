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

    let mut asteroids = create_asteroids(ship_pos);

    let mut hit = false;

    loop {
        clear_background(BLACK);

        let dt = get_frame_time();

        if is_key_pressed(KeyCode::Enter) {
            paused = !paused;
        }

        if !paused {
            // Position updates
            ship.update(dt);
            for a in &mut asteroids {
                a.update(dt);
            }

            // Collision detection

            // Asteroid <-> spacehsip collision
            hit = asteroids.iter().any(|a| {
                let (ship_pos, ship_rad) = ship.bounds();
                let (a_pos, a_rad) = a.bounds();
                circles_overlap_wrapped(ship_pos, ship_rad, a_pos, a_rad)
            });

            // Asteroid <-> asteroid collision

            // Detect
            let mut collisions: Vec<(usize, usize)> = Vec::new();
            for i in 0..asteroids.len() {
                let (a_i_pos, a_i_rad) = asteroids[i].bounds();
                for j in i + 1..asteroids.len() {
                    let (a_j_pos, a_j_rad) = asteroids[j].bounds();

                    if circles_overlap_wrapped(a_i_pos, a_i_rad, a_j_pos, a_j_rad) {
                        collisions.push((i, j));
                    }
                }
            }

            // Resolve
            for (i, j) in collisions {
                let [a_i, a_j] = asteroids
                    .get_disjoint_mut([i, j])
                    .expect("i and j are distinct");

                let (a_i_pos, a_i_rad) = a_i.bounds();
                let (a_j_pos, a_j_rad) = a_j.bounds();

                let delta = wrapped_delta(a_i_pos, a_j_pos);
                let dist = delta.length();
                let n = if dist < f32::EPSILON {
                    vec2(1.0, 0.0)
                } else {
                    delta / dist
                };

                let rel = a_i.velocity() - a_j.velocity();
                let along = rel.dot(n);
                if along > 0.0 {
                    a_i.set_velocity(a_i.velocity() - (along * n));
                    a_j.set_velocity(a_j.velocity() + (along * n));
                }

                let overlap = (a_i_rad + a_j_rad) - dist;
                let separation = n * (overlap * 0.5);
                a_i.set_position(a_i_pos - separation);
                a_j.set_position(a_j_pos + separation);
            }
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

        if paused && hit && is_key_down(KeyCode::Space) {
            ship = ship::Ship::new(ship_pos);
            asteroids = create_asteroids(ship_pos);
            hit = false;
            paused = false;
        }

        next_frame().await
    }
}

fn create_asteroids(ship_pos: Vec2) -> Vec<asteroid::Asteroid> {
    let mut asteroids = Vec::with_capacity(ASTEROID_COUNT);

    //                    ~ship_radius + buffer
    let asteroid_clearance = 20.0 + 150.0;

    for i in 0..ASTEROID_COUNT {
        let pos = asteroid::Asteroid::gen_position(ship_pos, asteroid_clearance, &asteroids);
        asteroids.push(asteroid::Asteroid::new(
            i as i32,
            asteroid::AsteroidSize::Large,
            pos,
        ));
    }
    asteroids
}

fn circles_overlap_wrapped(c1: Vec2, r1: f32, c2: Vec2, r2: f32) -> bool {
    wrapped_delta(c1, c2).length_squared() < (r1 + r2).powi(2)
}

fn wrapped_delta(p1: Vec2, p2: Vec2) -> Vec2 {
    let mut dx = p2.x - p1.x;
    if dx > (screen_width() / 2.0) {
        dx -= screen_width();
    } else if dx < -(screen_width() / 2.0) {
        dx += screen_width();
    }

    let mut dy = p2.y - p1.y;
    if dy > (screen_height() / 2.0) {
        dy -= screen_height();
    } else if dy < -(screen_height() / 2.0) {
        dy += screen_height();
    }

    vec2(dx, dy)
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
