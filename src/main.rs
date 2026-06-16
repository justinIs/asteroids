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
    let ship_pos = vec2(screen_width() / 2.0, screen_height() / 2.0);
    let mut ship = ship::Ship::new(ship_pos);

    //                    ~ship_radius + buffer
    let asteroid_clearance = 20.0 + 5.0;

    let mut asteroids: Vec<asteroid::Asteroid> = Vec::with_capacity(ASTEROID_COUNT);
    for _ in 0..ASTEROID_COUNT {
        let pos = asteroid::Asteroid::gen_position(ship_pos, asteroid_clearance, &asteroids);
        asteroids.push(asteroid::Asteroid::new(asteroid::AsteroidSize::Large, pos));
    }

    loop {
        clear_background(BLACK);

        let dt = get_frame_time();
        ship.update(dt);
        ship.draw();

        for a in &mut asteroids {
            a.update(dt);
            a.draw();
        }

        next_frame().await
    }
}
