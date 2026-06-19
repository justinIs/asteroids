use macroquad::prelude::*;

const MAX_BULLET_DISTANCE: f32 = 450.0;
const BULLET_SPEED: f32 = 150.0;

pub struct Bullet {
    position: Vec2,
    velocity: Vec2,
    distance_traveled: f32,
}

impl Bullet {
    pub fn new(position: Vec2, direction: Vec2) -> Bullet {
        let velocity = direction * BULLET_SPEED;
        Bullet {
            position,
            velocity,
            distance_traveled: 0.0,
        }
    }

    pub fn is_expired(&self) -> bool {
        self.distance_traveled >= MAX_BULLET_DISTANCE
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
        self.position.x = self.position.x.rem_euclid(screen_width());
        self.position.y = self.position.y.rem_euclid(screen_height());
        self.distance_traveled += step.length()
    }
}
