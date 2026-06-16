use macroquad::prelude::*;
use std::f32::consts::{PI, TAU};

use crate::vec_util;

const MAX_ASTERIOD_EDGES: u8 = 9;
const MIN_ASTEROID_EDGES: u8 = 5;
const ASTEROID_SPEED: f32 = 100.0;

pub enum AsteroidSize {
    Small,
    Medium,
    Large,
}

impl AsteroidSize {
    fn radius(&self) -> f32 {
        match self {
            AsteroidSize::Small => 7.5,
            AsteroidSize::Medium => 15.0,
            AsteroidSize::Large => 30.0,
        }
    }
}

pub struct Asteroid {
    size: AsteroidSize,
    position: Vec2,
    rotation: f32,
    verticies: Vec<Vec2>,
    spin: f32,
    direction: Vec2,
}

impl Asteroid {
    pub fn new(size: AsteroidSize, position: Vec2) -> Asteroid {
        let edges = rand::gen_range(MIN_ASTEROID_EDGES, MAX_ASTERIOD_EDGES + 1);
        let rotation = rand::gen_range(0.0, TAU);
        let verticies = Self::gen_verticies(&size, edges);
        let spin = rand::gen_range(-PI, PI);
        let direction = Self::gen_direction();

        Asteroid {
            size,
            position,
            rotation,
            verticies,
            spin,
            direction,
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

    pub fn gen_position(avoid_pos: Vec2, min_clearance: f32, other_asteroids: &[Asteroid]) -> Vec2 {
        loop {
            let candidate = vec2(
                rand::gen_range(0.0, screen_width()),
                rand::gen_range(0.0, screen_height()),
            );
            let mut overlaps_other = false;
            for other in other_asteroids {
                if candidate.distance(other.position)
                    < (other.size.radius() + AsteroidSize::Large.radius())
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
        let len = self.verticies.len();
        for i in 0..len {
            let start = self.position + vec_util::rotate(self.rotation, self.verticies[i]);
            let end =
                self.position + vec_util::rotate(self.rotation, self.verticies[(i + 1) % len]);

            draw_line(start.x, start.y, end.x, end.y, 1.5, WHITE);
        }
    }

    pub fn update(&mut self, dt: f32) {
        let velocity = self.direction * ASTEROID_SPEED;
        self.position += velocity * dt;

        if self.position.x > screen_width() {
            self.position.x = 0.0;
        } else if self.position.x < 0.0 {
            self.position.x = screen_width();
        }
        if self.position.y > screen_height() {
            self.position.y = 0.0;
        } else if self.position.y < 0.0 {
            self.position.y = screen_height();
        }

        self.rotation += self.spin * dt;
        self.rotation = self.rotation.rem_euclid(TAU);
    }
}
