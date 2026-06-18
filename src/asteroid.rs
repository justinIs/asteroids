use macroquad::prelude::*;
use std::f32::consts::{PI, TAU};

use crate::vec_util;

const MAX_ASTERIOD_EDGES: u8 = 9;
const MIN_ASTEROID_EDGES: u8 = 5;
const INITIAL_ASTEROID_SPEED: f32 = 50.0;

#[derive(Debug, Copy, Clone)]
pub enum AsteroidSize {
    Small,
    Medium,
    Large,
}

impl AsteroidSize {
    pub fn radius(&self) -> f32 {
        match self {
            AsteroidSize::Small => 7.5,
            AsteroidSize::Medium => 15.0,
            AsteroidSize::Large => 30.0,
        }
    }
}

pub struct Asteroid {
    id: i32,
    size: AsteroidSize,
    position: Vec2,
    rotation: f32,
    verticies: Vec<Vec2>,
    spin: f32,
    velocity: Vec2,
}

impl Asteroid {
    pub fn new(id: i32, size: AsteroidSize, position: Vec2) -> Asteroid {
        let edges = rand::gen_range(MIN_ASTEROID_EDGES, MAX_ASTERIOD_EDGES + 1);
        let rotation = rand::gen_range(0.0, TAU);
        let verticies = Self::gen_verticies(&size, edges);
        let spin = rand::gen_range(-PI, PI);
        let direction = Self::gen_direction();
        let velocity = direction * INITIAL_ASTEROID_SPEED;

        Asteroid {
            id,
            size,
            position,
            rotation,
            verticies,
            spin,
            velocity,
        }
    }

    fn gen_direction() -> Vec2 {
        let angle = rand::gen_range(0.0, TAU);
        vec2(angle.cos(), angle.sin())
    }

    fn gen_verticies(size: &AsteroidSize, edges: u8) -> Vec<Vec2> {
        let mut verticies: Vec<Vec2> = Vec::with_capacity(edges as usize);

        for i in 0..edges {
            let jitter = rand::gen_range(0.8, 1.2);
            let angle = (i as f32 / edges as f32) * TAU;
            let vertex = vec2(angle.cos(), angle.sin()) * size.radius() * jitter;

            verticies.push(vertex);
        }

        verticies
    }

    pub fn bounds(&self) -> (Vec2, f32) {
        (self.position, self.size.radius())
    }

    pub fn velocity(&self) -> Vec2 {
        self.velocity
    }

    pub fn set_velocity(&mut self, velocity: Vec2) {
        self.velocity = velocity;
    }

    pub fn size(&self) -> AsteroidSize {
        self.size
    }

    pub fn position(&self) -> Vec2 {
        self.position
    }

    pub fn set_position(&mut self, position: Vec2) {
        self.position = position;
        self.wrap_position();
    }

    pub fn gen_position(avoid_pos: Vec2, min_clearance: f32, other_asteroids: &[Asteroid]) -> Vec2 {
        loop {
            let candidate = vec2(
                rand::gen_range(0.0, screen_width()),
                rand::gen_range(0.0, screen_height()),
            );
            let mut overlaps_other = false;
            for other in other_asteroids {
                if candidate.distance(other.position)
                    < 2.0 * (other.size.radius() + AsteroidSize::Large.radius())
                {
                    overlaps_other = true;
                    break;
                }
            }
            if !overlaps_other && candidate.distance(avoid_pos) >= min_clearance {
                return candidate;
            }
        }
    }

    pub fn draw(&self) {
        let w = screen_width();
        let h = screen_height();
        let r = self.size.radius();
        let x = self.position.x;
        let y = self.position.y;

        let mut x_centers: Vec<f32> = vec![x];
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
        let len = self.verticies.len();

        for i in 0..len {
            let start = center + vec_util::rotate(self.rotation, self.verticies[i]);
            let end = center + vec_util::rotate(self.rotation, self.verticies[(i + 1) % len]);

            draw_line(start.x, start.y, end.x, end.y, 1.5, WHITE);
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.position += self.velocity * dt;
        self.wrap_position();

        self.rotation += self.spin * dt;
        self.rotation = self.rotation.rem_euclid(TAU);
    }

    fn wrap_position(&mut self) {
        self.position.x = self.position.x.rem_euclid(screen_width());

        self.position.y = self.position.y.rem_euclid(screen_height());
    }
}
