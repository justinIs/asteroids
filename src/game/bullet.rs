use macroquad::prelude::*;

use crate::layout;

const MAX_BULLET_DISTANCE_RATIO: f32 = 0.65;
const BULLET_SPEED: f32 = 250.0;

pub struct Bullet {
    position: Vec2,
    velocity: Vec2,
    distance_traveled: f32,
    last_step: Vec2,
}

impl Bullet {
    pub fn new(position: Vec2, direction: Vec2, inherited: Vec2) -> Bullet {
        let velocity = inherited + direction * BULLET_SPEED;
        Bullet {
            position,
            velocity,
            distance_traveled: 0.0,
            last_step: Vec2::ZERO,
        }
    }

    fn max_bullet_distance() -> f32 {
        MAX_BULLET_DISTANCE_RATIO * layout::WORLD_H.max(layout::WORLD_W)
    }

    pub fn position(&self) -> Vec2 {
        self.position
    }

    pub fn is_expired(&self) -> bool {
        self.distance_traveled >= Self::max_bullet_distance()
    }

    pub fn last_step(&self) -> Vec2 {
        self.last_step
    }

    pub fn draw(&self) {
        draw_rectangle(
            self.position.x - 1.5,
            self.position.y - 1.5,
            3.0,
            3.0,
            WHITE,
        );
    }

    pub fn update(&mut self, dt: f32) {
        let step = self.velocity * dt;
        self.position += step;
        self.position.x = self.position.x.rem_euclid(layout::WORLD_W);
        self.position.y = self.position.y.rem_euclid(layout::WORLD_H);
        self.distance_traveled += step.abs().length();
        self.last_step = step;
    }
}
