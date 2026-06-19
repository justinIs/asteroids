use macroquad::prelude::*;

use crate::{game::Game, screen::Screen, transition::Transition};

mod asteroid;
mod bullet;
mod camera;
mod controls;
mod game;
mod layout;
mod screen;
mod ship;
mod transition;
mod vec_util;

fn window_conf() -> Conf {
    Conf {
        window_title: "Asteroids".to_owned(),
        window_width: 900,
        window_height: 900,
        window_resizable: false,
        high_dpi: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    println!("screen: {}, {}", screen_width(), screen_height());

    let mut screen = Screen::Start;

    loop {
        clear_background(BLACK);

        let c = read_controls();

        set_camera(&camera::world_camera());

        let dt = get_frame_time();

        let transition = match &mut screen {
            Screen::Start => {
                if c.any_press() {
                    Transition::NewGame
                } else {
                    Transition::None
                }
            }
            Screen::Playing(game) => game.update(dt, &c),
            Screen::GameOver { score } => {
                if c.any_press() {
                    Transition::ToStart
                } else {
                    Transition::None
                }
            }
        };

        match &screen {
            Screen::Start => draw_start(),
            Screen::Playing(game) => game.draw(),
            Screen::GameOver { score } => draw_game_over(*score),
        }

        set_default_camera();
        if c.using_touch && matches!(screen, Screen::Playing(_)) {
            draw_touch_buttons();
        }

        match transition {
            Transition::None => {}
            Transition::NewGame => screen = Screen::Playing(Game::new()),
            Transition::ToStart => screen = Screen::Start,
            Transition::GameOver { score } => {
                screen = Screen::GameOver { score };
            }
        }

        // if is_key_pressed(KeyCode::Enter) {
        //     paused = !paused;
        // }

        // if !paused {
        //     // Position updates
        //     ship.update(dt, &c);
        //     for a in &mut asteroids {
        //         a.update(dt);
        //     }
        //     for b in &mut bullets {
        //         b.update(dt);
        //     }
        //     bullets.retain(|b| !b.is_expired());

        //     // Shots fired
        //     if fire {
        //         let (pos, dir) = ship.muzzle();
        //         bullets.push(bullet::Bullet::new(pos, dir));
        //     }

        //     // Collision detection

        //     // Asteroid <-> spacehsip collision
        //     crashed = asteroids.iter().any(|a| ship.collides_with(a));

        //     // Asteroid <-> asteroid collision
        //     let collisions = asteroid::Asteroid::find_collisions(&asteroids);
        //     for (i, j) in collisions {
        //         let [a, b] = asteroids
        //             .get_disjoint_mut([i, j])
        //             .expect("i and j are distinct");

        //         a.collide_with(b);
        //     }

        //     // Asteroid <-> bullet collision
        //     let mut hit_asteroids = vec![false; asteroids.len()];
        //     bullets.retain(|b| {
        //         match asteroids.iter().position(|a| {
        //             let (a_pos, a_rad) = a.bounds();
        //             // TODO: handle double it (if the asteroid being checkd here already was hit it
        //             // would eat a bullet for no reason)
        //             vec_util::circles_overlap_wrapped(b.position(), 0.0, a_pos, a_rad)
        //         }) {
        //             Some(i) => {
        //                 hit_asteroids[i] = true;
        //                 false
        //             }
        //             None => true,
        //         }
        //     });

        //     for (i, a) in std::mem::take(&mut asteroids).into_iter().enumerate() {
        //         if hit_asteroids[i] {
        //             asteroids.extend(a.split());
        //         } else {
        //             asteroids.push(a);
        //         }
        //     }
        // }
        // if crashed {
        //     paused = true;
        // }

        // ship.draw();
        // for a in &asteroids {
        //     a.draw();
        // }
        // for b in &bullets {
        //     b.draw();
        // }

        // if crashed && paused {
        //     draw_centered_outlined("HIT", 80.0, WHITE, BLACK);
        // }

        // if paused && crashed && is_key_down(KeyCode::Enter) {
        //     ship = ship::Ship::new(ship_pos);
        //     asteroids = create_asteroids(ship_pos);
        //     bullets.clear();
        //     crashed = false;
        //     paused = false;
        // }

        // set_default_camera();

        // draw_touch_buttons();

        next_frame().await;
    }
}

fn draw_centered_outlined(text: &str, font_size: f32, fill: Color, outline: Color) {
    let dims = measure_text(text, None, font_size as u16, 1.0);
    let x = (layout::WORLD_W - dims.width) / 2.0;
    let y = (layout::WORLD_H - dims.height) / 2.0;

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

fn draw_start() {}

fn draw_game_over(score: u32) {}

// Touch HUD

#[derive(Clone, Copy)]
enum Action {
    RotateLeft,
    RotateRight,
    Thrust,
    Fire,
}

struct Button {
    rect: Rect,
    label: &'static str,
    action: Action,
}

fn button_layout() -> [Button; 4] {
    let (w, h) = (screen_width(), screen_height());
    let s = 90.0; // button size
    let m = 28.0; // margin
    let y = h - s - m;
    [
        Button {
            rect: Rect::new(m, y, s, s),
            label: "<",
            action: Action::RotateLeft,
        },
        Button {
            rect: Rect::new(m + s + 16.0, y, s, s),
            label: ">",
            action: Action::RotateRight,
        },
        Button {
            rect: Rect::new(w - s - m, y - s - m, s, s),
            label: "^",
            action: Action::Thrust,
        },
        Button {
            rect: Rect::new(w - s - m, y, s, s),
            label: "0",
            action: Action::Fire,
        },
    ]
}

fn pointers() -> Vec<Vec2> {
    let mut ps: Vec<Vec2> = touches().iter().map(|t| t.position).collect();

    if is_mouse_button_down(MouseButton::Left) {
        ps.push(mouse_position().into());
    }

    ps
}

fn read_controls() -> controls::Controls {
    let ps = pointers();
    let mut c = controls::Controls {
        ship_controls: ship::ShipControls {
            rotate_left: is_key_down(KeyCode::Left) || is_key_down(KeyCode::A),
            rotate_right: is_key_down(KeyCode::Right) || is_key_down(KeyCode::D),
            thrust: is_key_down(KeyCode::Up) || is_key_down(KeyCode::W),
        },
        pause: is_key_pressed(KeyCode::Enter),
        fire: is_key_pressed(KeyCode::Space),
        using_touch: false,
    };

    for b in button_layout() {
        if ps.iter().any(|p| b.rect.contains(*p)) {
            match b.action {
                Action::RotateLeft => c.ship_controls.rotate_left = true,
                Action::RotateRight => c.ship_controls.rotate_right = true,
                Action::Thrust => c.ship_controls.thrust = true,
                Action::Fire => c.fire = true,
            }
        }
    }
    c
}

fn draw_touch_buttons() {
    let ps = pointers();
    for b in button_layout() {
        let pressed = ps.iter().any(|p| b.rect.contains(*p));
        let fill = if pressed {
            Color::new(1.0, 1.0, 1.0, 0.35)
        } else {
            Color::new(1.0, 1.0, 1.0, 0.12)
        };

        draw_rectangle(b.rect.x, b.rect.y, b.rect.w, b.rect.h, fill);
        draw_rectangle_lines(b.rect.x, b.rect.y, b.rect.w, b.rect.h, 2.0, WHITE);
        draw_text(
            b.label,
            b.rect.x + b.rect.w / 2.0 - 8.0,
            b.rect.y + b.rect.h / 2.0 + 8.0,
            36.0,
            WHITE,
        );
    }
}
