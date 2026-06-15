use macroquad::prelude::*;

mod ship;

fn window_conf() -> Conf {
    Conf {
        window_title: "Asteroids".to_owned(),
        window_width: 900,
        window_height: 900,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut ship = ship::Ship::new();

    loop {
        clear_background(BLACK);

        ship.update(get_frame_time());
        ship.draw();

        next_frame().await
    }
}
