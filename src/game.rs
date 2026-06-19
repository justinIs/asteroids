use crate::{asteroid, bullet, controls, layout, ship, transition, vec_util};
use macroquad::prelude::*;

const ASTEROID_COUNT: usize = 5;

pub struct Game {
    ship: ship::Ship,
    asteroids: Vec<asteroid::Asteroid>,
    bullets: Vec<bullet::Bullet>,
    score: u32,
    paused: bool,
}

impl Game {
    pub fn new() -> Self {
        let ship_pos = vec2(layout::WORLD_W / 2.0, layout::WORLD_H / 2.0);
        let ship = ship::Ship::new(ship_pos);

        let asteroids = Self::create_asteroids(ship_pos, ASTEROID_COUNT);
        let bullets: Vec<bullet::Bullet> = Vec::new();
        let score: u32 = 0;
        let paused = false;

        Self {
            ship,
            asteroids,
            bullets,
            score,
            paused,
        }
    }

    pub fn update(&mut self, dt: f32, c: &controls::Controls) -> transition::Transition {
        if !c.pause {
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
            if c.fire {
                let (pos, dir) = self.ship.muzzle();
                self.bullets.push(bullet::Bullet::new(pos, dir));
            }

            // Collision detection

            // Asteroid <-> spaceship
            let crashed = self.asteroids.iter().any(|a| self.ship.collides_with(a));
            if crashed {
                return transition::Transition::GameOver { score: self.score };
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
            let mut hit_asteroids = vec![false; self.asteroids.len()];
            self.bullets.retain(|b| {
                match self.asteroids.iter().position(|a| {
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

            for (i, a) in std::mem::take(&mut self.asteroids).into_iter().enumerate() {
                if hit_asteroids[i] {
                    self.asteroids.extend(a.split());
                } else {
                    self.asteroids.push(a);
                }
            }
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
    }

    fn create_asteroids(avoid_pos: Vec2, num_asteroids: usize) -> Vec<asteroid::Asteroid> {
        let mut asteroids = Vec::with_capacity(num_asteroids);

        //                    ~ship_radius + buffer
        let asteroid_clearance = 20.0 + 150.0;

        for _ in 0..ASTEROID_COUNT {
            let pos = asteroid::Asteroid::gen_position(avoid_pos, asteroid_clearance, &asteroids);
            asteroids.push(asteroid::Asteroid::new(asteroid::AsteroidSize::Large, pos));
        }
        asteroids
    }
}
