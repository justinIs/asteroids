use macroquad::prelude::*;

mod asteroid;
mod bullet;
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
    println!("screen: {}, {}", screen_width(), screen_height());
    let mut paused = false;

    let ship_pos = vec2(screen_width() / 2.0, screen_height() / 2.0);
    let mut ship = ship::Ship::new(ship_pos);

    let mut asteroids = create_asteroids(ship_pos);
    let mut bullets: Vec<bullet::Bullet> = Vec::new();

    let mut crashed = false;

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
            for b in &mut bullets {
                b.update(dt);
            }
            bullets.retain(|b| !b.is_expired());

            // Shots fired
            if is_key_pressed(KeyCode::Space) {
                let (pos, dir) = ship.muzzle();
                bullets.push(bullet::Bullet::new(pos, dir));
            }

            // Collision detection

            // Asteroid <-> spacehsip collision
            crashed = asteroids.iter().any(|a| ship.collides_with(a));

            // Asteroid <-> asteroid collision
            let collisions = asteroid::Asteroid::find_collisions(&asteroids);
            for (i, j) in collisions {
                let [a, b] = asteroids
                    .get_disjoint_mut([i, j])
                    .expect("i and j are distinct");

                a.collide_with(b);
            }

            // Asteroid <-> bullet collision
            let mut hit_asteroids = vec![false; asteroids.len()];
            bullets.retain(|b| {
                match asteroids.iter().position(|a| {
                    let (a_pos, a_rad) = a.bounds();
                    // TODO: handle double it (if the asteroid being checkd here already was hit it
                    // would eat a bullet for no reason)
                    vec_util::circles_overlap_wrapped(b.position(), 0.0, a_pos, a_rad)
                }) {
                    Some(i) => {
                        hit_asteroids[i] = true;
                        false
                    }
                    None => true,
                }
            });

            for (i, a) in std::mem::take(&mut asteroids).into_iter().enumerate() {
                if hit_asteroids[i] {
                    asteroids.extend(a.split());
                } else {
                    asteroids.push(a);
                }
            }
        }
        if crashed {
            paused = true;
        }

        ship.draw();
        for a in &asteroids {
            a.draw();
        }
        for b in &bullets {
            b.draw();
        }

        if crashed && paused {
            draw_centered_outlined("HIT", 80.0, WHITE, BLACK);
        }

        if paused && crashed && is_key_down(KeyCode::Space) {
            ship = ship::Ship::new(ship_pos);
            asteroids = create_asteroids(ship_pos);
            bullets.clear();
            crashed = false;
            paused = false;
        }

        next_frame().await
    }
}

fn create_asteroids(ship_pos: Vec2) -> Vec<asteroid::Asteroid> {
    let mut asteroids = Vec::with_capacity(ASTEROID_COUNT);

    //                    ~ship_radius + buffer
    let asteroid_clearance = 20.0 + 150.0;

    for _ in 0..ASTEROID_COUNT {
        let pos = asteroid::Asteroid::gen_position(ship_pos, asteroid_clearance, &asteroids);
        asteroids.push(asteroid::Asteroid::new(asteroid::AsteroidSize::Large, pos));
    }
    asteroids
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
