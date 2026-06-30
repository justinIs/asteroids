use crate::{input, layout, text, transition};
use macroquad::prelude::*;

mod asteroid;
mod bullet;
mod ship;

const SCORE_INC: u32 = 10;

#[derive(Default)]
pub struct GameControls {
    using_touch: bool,
    ship_controls: ship::ShipControls,
    fire: bool,
    pause: bool,
    advance: bool,

    touch_buttons: Option<[Button; 5]>,
}

pub struct Game {
    ship: ship::Ship,
    level: usize,
    level_time: f32,   // seconds elapsed in the current level
    next_event: usize, // next event to fire/how many events have already fired
    asteroids: Vec<asteroid::Asteroid>,
    bullets: Vec<bullet::Bullet>,
    score: u32,
    paused: bool,
    cleared: bool,
    controls: GameControls,
}

impl Game {
    pub fn new() -> Self {
        let ship_pos = vec2(layout::WORLD_W / 2.0, layout::WORLD_H / 2.0);
        let ship = ship::Ship::new(ship_pos);

        let asteroids = Self::create_asteroids(ship_pos, LEVELS[0].initial_asteroids);
        let bullets: Vec<bullet::Bullet> = Vec::new();
        let score: u32 = 0;
        let paused = false;
        let cleared = false;
        let controls = GameControls::default();

        Self {
            ship,
            level: 0,
            level_time: 0.0,
            next_event: 0,
            asteroids,
            bullets,
            score,
            paused,
            cleared,
            controls,
        }
    }

    pub fn score(&self) -> u32 {
        self.score
    }

    pub fn update(&mut self, dt: f32, input: &mut input::Input) -> transition::Transition {
        self.update_controls(input);
        let c = &self.controls;

        if self.cleared && (c.pause || c.advance) {
            match LEVELS.get(self.level + 1) {
                Some(spec) => {
                    self.level += 1;
                    self.ship
                        .reset(vec2(layout::WORLD_W / 2.0, layout::WORLD_H / 2.0));
                    self.asteroids =
                        Self::create_asteroids(self.ship.bounds().0, spec.initial_asteroids);
                    self.level_time = 0.0;
                    self.next_event = 0;

                    self.bullets.clear();
                }
                None => return transition::Transition::GameOver(true),
            }
            self.cleared = false;
        } else if c.pause {
            self.paused = !self.paused;
        }
        if !self.paused && !self.cleared {
            self.level_time += dt;
            let Some(spec) = LEVELS.get(self.level) else {
                // should never happen but return game over in case
                return transition::Transition::GameOver(true);
            };

            // Events
            if let Some(event) = spec.events.get(self.next_event)
                && event.at <= self.level_time
            {
                match event.spawn {
                    Spawn::Asteroids(num) => self
                        .asteroids
                        .extend(Self::create_asteroids(self.ship.bounds().0, num)),
                    Spawn::Alien => (),
                }
                self.next_event += 1;
            }

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
                && let Some((pos, dir, velocity)) = self.ship.try_fire()
            {
                self.bullets.push(bullet::Bullet::new(pos, dir, velocity));
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
                match self
                    .asteroids
                    .iter()
                    .enumerate()
                    .find(|(i, a)| {
                        !hit_asteroids[*i] && a.intersects_segment(b.position(), b.last_step())
                    })
                    .map(|(i, _)| i)
                {
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

            self.cleared = self.asteroids.is_empty() && spec.events.get(self.next_event).is_none();
            if self.cleared {
                input.consume();
            }
        }
        transition::Transition::None
    }

    fn update_controls(&mut self, i: &input::Input) {
        self.controls.using_touch = i.using_touch;
        self.controls.ship_controls.rotate_left =
            i.is_key_down(KeyCode::Left) || i.is_key_down(KeyCode::A);
        self.controls.ship_controls.rotate_right =
            i.is_key_down(KeyCode::Right) || i.is_key_down(KeyCode::D);
        self.controls.ship_controls.thrust =
            i.is_key_down(KeyCode::Up) || i.is_key_down(KeyCode::W);
        self.controls.pause = is_key_pressed(KeyCode::Enter);
        self.controls.fire = i.is_key_down(KeyCode::Space);
        self.controls.advance = false;

        if i.using_touch {
            if self.controls.touch_buttons.is_none() {
                self.controls.touch_buttons = Some(button_layout());
            }
            for b in self.controls.touch_buttons.as_ref().unwrap() {
                if i.is_pressed(b.rect) {
                    match b.action {
                        Action::RotateLeft => self.controls.ship_controls.rotate_left = true,
                        Action::RotateRight => self.controls.ship_controls.rotate_right = true,
                        Action::Thrust => self.controls.ship_controls.thrust = true,
                        Action::Fire => self.controls.fire = true,
                        Action::Advance => self.controls.advance = true,
                    }
                }
            }
        }
    }

    pub fn draw(&self) {
        self.ship.draw();
        for a in &self.asteroids {
            a.draw();
        }
        for b in &self.bullets {
            b.draw();
        }

        self.draw_level();
        self.draw_score();

        if self.cleared {
            self.draw_centered("Cleared!");
        } else if self.paused {
            self.draw_centered("Paused");
        }
    }

    fn draw_level(&self) {
        let text = format!("Level: {}", self.level + 1);
        let size = 30.0;
        let dims = measure_text(&text, None, size as u16, 1.0);
        let margin = 12.0;
        let x = margin;
        let y = margin + dims.height;
        draw_text(&text, x, y, size, WHITE);
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

    pub fn draw_touch_buttons(&self, i: &input::Input) {
        if let Some(buttons) = self.controls.touch_buttons.as_ref() {
            for b in buttons {
                if b.action == Action::Advance && !self.cleared {
                    continue;
                }
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
    }
    fn draw_centered(&self, text: &str) {
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

enum Spawn {
    Asteroids(usize),
    Alien,
}

struct SpawnEvent {
    at: f32, // seconds since level started
    spawn: Spawn,
}

struct LevelSpec {
    initial_asteroids: usize,
    events: &'static [SpawnEvent],
}

const LEVELS: &[LevelSpec] = &[
    // Level 1
    LevelSpec {
        initial_asteroids: 1,
        events: &[
            SpawnEvent {
                at: 15.0,
                spawn: Spawn::Asteroids(1),
            },
            SpawnEvent {
                at: 30.0,
                spawn: Spawn::Asteroids(1),
            },
        ],
    },
    // Level 2
    LevelSpec {
        initial_asteroids: 5,
        events: &[SpawnEvent {
            at: 0.0,
            spawn: Spawn::Alien,
        }],
    },
    // Level 3
    LevelSpec {
        initial_asteroids: 5,
        events: &[
            SpawnEvent {
                at: 0.0,
                spawn: Spawn::Alien,
            },
            SpawnEvent {
                at: 15.0,
                spawn: Spawn::Asteroids(2),
            },
        ],
    },
];

#[derive(Clone, Copy, Eq, PartialEq)]
enum Action {
    RotateLeft,
    RotateRight,
    Thrust,
    Fire,
    Advance,
}

struct Button {
    rect: Rect,
    label: &'static str,
    action: Action,
}

fn button_layout() -> [Button; 5] {
    let (w, h) = (screen_width(), screen_height());
    let s = 90.0 * layout::ui_scale(); // button size
    let m = 28.0 * layout::ui_scale(); // margin
    let g = 16.0 * layout::ui_scale(); // gap
    let y = h - s - m;

    let advance = "advance";
    let advance_font_size = 36.0 * layout::ui_scale();
    let advance_dims = measure_text(advance, None, advance_font_size as u16, 1.0);
    let advance_x = (w - advance_dims.width) / 2.0 + 10.0;

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
        Button {
            rect: Rect::new(
                advance_x,
                y,
                20.0 + advance_dims.width,
                10.0 + advance_dims.height,
            ),
            label: advance,
            action: Action::Advance,
        },
    ]
}
