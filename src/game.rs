use crate::{input, layout, text, transition};
use macroquad::prelude::*;

mod asteroid;
mod bullet;
mod ship;

const ASTEROID_COUNT: usize = 5;
const SCORE_INC: u32 = 10;

pub struct GameControls {
    pub ship_controls: ship::ShipControls,
    pub fire: bool,
    pub pause: bool,
}

impl GameControls {
    pub fn from_input(i: &input::Input) -> Self {
        let mut c = Self {
            ship_controls: ship::ShipControls {
                rotate_left: i.is_key_down(KeyCode::Left) || i.is_key_down(KeyCode::A),
                rotate_right: i.is_key_down(KeyCode::Right) || i.is_key_down(KeyCode::D),
                thrust: i.is_key_down(KeyCode::Up) || i.is_key_down(KeyCode::W),
            },
            pause: is_key_pressed(KeyCode::Enter),
            fire: i.is_key_down(KeyCode::Space),
        };

        if i.using_touch {
            for b in button_layout() {
                if i.is_pressed(b.rect) {
                    match b.action {
                        Action::RotateLeft => c.ship_controls.rotate_left = true,
                        Action::RotateRight => c.ship_controls.rotate_right = true,
                        Action::Thrust => c.ship_controls.thrust = true,
                        Action::Fire => c.fire = true,
                    }
                }
            }
        }

        c
    }
}

pub struct Game {
    ship: ship::Ship,
    asteroids: Vec<asteroid::Asteroid>,
    bullets: Vec<bullet::Bullet>,
    score: u32,
    paused: bool,
    cleared: bool,
}

impl Game {
    pub fn new() -> Self {
        let ship_pos = vec2(layout::WORLD_W / 2.0, layout::WORLD_H / 2.0);
        let ship = ship::Ship::new(ship_pos);

        let asteroids = Self::create_asteroids(ship_pos, ASTEROID_COUNT);
        let bullets: Vec<bullet::Bullet> = Vec::new();
        let score: u32 = 0;
        let paused = false;
        let cleared = false;

        Self {
            ship,
            asteroids,
            bullets,
            score,
            paused,
            cleared,
        }
    }

    pub fn score(&self) -> u32 {
        self.score
    }

    pub fn update(&mut self, dt: f32, c: &GameControls) -> transition::Transition {
        if c.pause {
            self.paused = !self.paused;
        }
        if self.cleared && (c.fire || c.pause) {
            return transition::Transition::GameOver(true);
        }
        if !self.paused && !self.cleared {
            // Position update
            self.ship.update(dt, &c.ship_controls);

            for a in &mut self.asteroids {
                a.update(dt);
            }
            for b in &mut self.bullets {
                b.update(dt);
            }
            self.bullets.retain(|b| !b.is_expired());

            // Shots fired
            if c.fire
                && let Some((pos, dir)) = self.ship.try_fire()
            {
                self.bullets.push(bullet::Bullet::new(pos, dir));
            }

            // Collision detection

            // Asteroid <-> spaceship
            let crashed = self.asteroids.iter().any(|a| self.ship.collides_with(a));
            if crashed {
                return transition::Transition::GameOver(false);
            }

            // Asteroid <-> asteroid
            let collisions = asteroid::Asteroid::find_collisions(&self.asteroids);
            for (i, j) in collisions {
                let [a, b] = self
                    .asteroids
                    .get_disjoint_mut([i, j])
                    .expect("i and j are distinct");

                a.collide_with(b);
            }

            // Asteroid <-> bullet

            // First detect hit asteroids
            let mut hit_asteroids = vec![false; self.asteroids.len()];
            self.bullets.retain(|b| {
                match self.asteroids.iter().position(|a| {
                    // TODO: handle double it (if the asteroid being checkd here already was hit it
                    // would eat a bullet for no reason)
                    // TODO: consider doing a swept test to handle bullet skipping through smaller
                    // asteroids or edges
                    a.contains_point(b.position())
                }) {
                    Some(i) => {
                        hit_asteroids[i] = true;
                        self.score += SCORE_INC;
                        false
                    }
                    None => true,
                }
            });

            // Then rebuild self.asteroids by splitting hit asteroids and pushing the others
            for (i, a) in std::mem::take(&mut self.asteroids).into_iter().enumerate() {
                if hit_asteroids[i] {
                    self.asteroids.extend(a.split());
                } else {
                    self.asteroids.push(a);
                }
            }

            self.cleared = self.asteroids.is_empty();
        }
        transition::Transition::None
    }

    pub fn draw(&self) {
        self.ship.draw();
        for a in &self.asteroids {
            a.draw();
        }
        for b in &self.bullets {
            b.draw();
        }

        self.draw_score();

        if self.cleared {
            self.draw_cleared();
        }
    }

    fn draw_score(&self) {
        let text = format!("{}", self.score);
        let size = 30.0;
        let dims = measure_text(&text, None, size as u16, 1.0);
        let margin = 12.0;
        let x = layout::WORLD_W - dims.width - margin;
        let y = margin + dims.height;
        draw_text(&text, x, y, size, WHITE);
    }

    fn draw_cleared(&self) {
        let text = "Cleared!";
        let size = 80.0;
        let dims = measure_text(text, None, size as u16, 1.0);
        let x = (layout::WORLD_W - dims.width) / 2.0;
        let y = (layout::WORLD_H - dims.height) / 2.0;

        text::draw_outlined(text, x, y, size, WHITE, BLACK);
    }

    fn create_asteroids(avoid_pos: Vec2, num_asteroids: usize) -> Vec<asteroid::Asteroid> {
        let mut asteroids = Vec::with_capacity(num_asteroids);

        //                    ~ship_radius + buffer
        let asteroid_clearance = 20.0 + 150.0;

        for _ in 0..num_asteroids {
            let pos = asteroid::Asteroid::gen_position(avoid_pos, asteroid_clearance, &asteroids);
            asteroids.push(asteroid::Asteroid::new(asteroid::AsteroidSize::Large, pos));
        }
        asteroids
    }
}

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
    let s = 90.0 * layout::ui_scale(); // button size
    let m = 28.0 * layout::ui_scale(); // margin
    let g = 16.0 * layout::ui_scale(); // gap
    let y = h - s - m;
    [
        Button {
            rect: Rect::new(m, y, s, s),
            label: "<",
            action: Action::RotateLeft,
        },
        Button {
            rect: Rect::new(m + s + g, y, s, s),
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

pub fn draw_touch_buttons(i: &input::Input) {
    for b in button_layout() {
        let pressed = i.is_pressed(b.rect);
        let fill = if pressed {
            Color::new(1.0, 1.0, 1.0, 0.35)
        } else {
            Color::new(1.0, 1.0, 1.0, 0.12)
        };

        draw_rectangle(b.rect.x, b.rect.y, b.rect.w, b.rect.h, fill);
        draw_rectangle_lines(
            b.rect.x,
            b.rect.y,
            b.rect.w,
            b.rect.h,
            2.0 * layout::ui_scale(),
            WHITE,
        );
        draw_text(
            b.label,
            b.rect.x + b.rect.w / 2.0 - 8.0,
            b.rect.y + b.rect.h / 2.0 + 8.0,
            36.0 * layout::ui_scale(),
            WHITE,
        );
    }
}
