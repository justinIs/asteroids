use macroquad::prelude::*;

use crate::{asteroid::Asteroid, layout, vec_util};

const ROTATION_SPEED: f32 = std::f32::consts::PI + 1.5;
const THRUST: f32 = 100.0;
const MAX_VELOCITY: f32 = 200.0;

pub struct Controls {
    pub rotate_left: bool,
    pub rotate_right: bool,
    pub thrust: bool,
}

pub struct Ship {
    position: Vec2,
    velocity: Vec2,
    rotation: f32,
}

impl Ship {
    pub fn new(position: Vec2) -> Ship {
        Ship {
            position,
            velocity: Vec2::ZERO,
            rotation: 0.0,
        }
    }

    pub fn bounds(&self) -> (Vec2, f32) {
        (self.position, Self::SHIP_POINTS[0].y.abs())
    }

    pub fn muzzle(&self) -> (Vec2, Vec2) {
        let direction = vec2(self.rotation.sin(), -self.rotation.cos());
        let nos_pos = self.position + direction * Self::SHIP_POINTS[0].y.abs();

        (nos_pos, direction)
    }

    pub fn update(&mut self, dt: f32, controls: &Controls) {
        // Handle rotation
        // let right = is_key_down(KeyCode::Right) || is_key_down(KeyCode::D);
        // let left = is_key_down(KeyCode::Left) || is_key_down(KeyCode::A);
        if controls.rotate_right && !controls.rotate_left {
            self.rotation += ROTATION_SPEED * dt;
        } else if controls.rotate_left && !controls.rotate_right {
            self.rotation -= ROTATION_SPEED * dt;
        }
        self.rotation = self.rotation.rem_euclid(std::f32::consts::TAU);

        // Handle thrust
        // let up = is_key_down(KeyCode::Up) || is_key_down(KeyCode::W);
        if controls.thrust {
            let direction = vec2(self.rotation.sin(), -self.rotation.cos());
            self.velocity += direction * THRUST * dt;
            self.velocity = self.velocity.clamp_length_max(MAX_VELOCITY);
        }

        // Update position based on velocity
        self.position += self.velocity * dt;
        if self.position.x > layout::WORLD_W {
            self.position.x = 0.0;
        } else if self.position.x < 0.0 {
            self.position.x = layout::WORLD_W;
        }
        if self.position.y > layout::WORLD_H {
            self.position.y = 0.0;
        } else if self.position.y < 0.0 {
            self.position.y = layout::WORLD_H;
        }
    }

    // Dimensions for the ship
    const SHIP_POINTS: [Vec2; 4] = [
        // Nose
        vec2(0.0, -18.0),
        // Right rear
        vec2(12.0, 12.0),
        // Bottom
        vec2(0.0, 5.0),
        // Left rear
        vec2(-12.0, 12.0),
    ];

    pub fn draw(&self) {
        let w = layout::WORLD_W;
        let h = layout::WORLD_H;
        let x = self.position.x;
        let y = self.position.y;

        // Use bounding circle with radius of distance from center to farthest vertex (nose)
        let r = Self::SHIP_POINTS[0].y.abs();

        let mut x_centers = vec![x];
        if x + r > w {
            x_centers.push(x - w);
        } else if x - r < 0.0 {
            x_centers.push(x + w);
        }

        let mut y_centers = vec![y];
        if y + r > h {
            y_centers.push(y - h);
        } else if y - r < 0.0 {
            y_centers.push(y + h);
        }

        for x in &x_centers {
            for y in &y_centers {
                self.draw_at(vec2(*x, *y));
            }
        }
    }

    fn draw_at(&self, center: Vec2) {
        let rotated = Self::SHIP_POINTS.map(|p| vec_util::rotate(self.rotation, p) + center);

        for i in 0..rotated.len() {
            let p1 = rotated[i];
            let p2 = rotated[(i + 1) % rotated.len()];

            draw_line(p1.x, p1.y, p2.x, p2.y, 1.5, WHITE);
        }
    }

    pub fn collides_with(&self, a: &Asteroid) -> bool {
        let (ship_pos, ship_rad) = self.bounds();
        let (a_pos, a_rad) = a.bounds();
        vec_util::circles_overlap_wrapped(ship_pos, ship_rad, a_pos, a_rad)
    }
}
