use macroquad::prelude::*;
use std::f32::consts::{PI, TAU};

use crate::{layout, vec_util};

const MAX_ASTERIOD_EDGES: u8 = 9;
const MIN_ASTEROID_EDGES: u8 = 5;

// Spin gain coefficient
const K: f32 = 1.5;

#[derive(Debug, Copy, Clone)]
pub enum AsteroidSize {
    Small,
    Medium,
    Large,
}

impl AsteroidSize {
    pub fn radius(&self) -> f32 {
        match self {
            AsteroidSize::Small => 10.0,
            AsteroidSize::Medium => 20.0,
            AsteroidSize::Large => 40.0,
        }
    }

    fn initial_speed(&self) -> f32 {
        match self {
            AsteroidSize::Small => 70.0,
            AsteroidSize::Medium => 60.0,
            AsteroidSize::Large => 50.0,
        }
    }

    fn mass(&self) -> f32 {
        match self {
            AsteroidSize::Small => 1.0,
            AsteroidSize::Medium => 4.0,
            AsteroidSize::Large => 16.0,
        }
    }

    fn smaller(&self) -> Option<AsteroidSize> {
        match self {
            AsteroidSize::Large => Some(AsteroidSize::Medium),
            AsteroidSize::Medium => Some(AsteroidSize::Small),
            AsteroidSize::Small => None,
        }
    }
}

pub struct Asteroid {
    size: AsteroidSize,
    position: Vec2,
    rotation: f32,
    verticies: Vec<Vec2>,
    bounding_radius: f32,
    spin: f32,
    velocity: Vec2,
}

impl Asteroid {
    pub fn new(size: AsteroidSize, position: Vec2) -> Asteroid {
        let edges = rand::gen_range(MIN_ASTEROID_EDGES, MAX_ASTERIOD_EDGES + 1);
        let rotation = rand::gen_range(0.0, TAU);
        let verticies = Self::gen_verticies(&size, edges);
        let bounding_radius = verticies.iter().map(|v| v.length()).fold(0.0_f32, f32::max);
        let spin = rand::gen_range(-PI, PI);
        let direction = Self::gen_direction();
        let velocity = direction * size.initial_speed();

        Asteroid {
            size,
            position,
            rotation,
            verticies,
            bounding_radius,
            spin,
            velocity,
        }
    }

    fn with_velocity(size: AsteroidSize, position: Vec2, velocity: Vec2) -> Asteroid {
        let edges = rand::gen_range(MIN_ASTEROID_EDGES, MAX_ASTERIOD_EDGES + 1);
        let rotation = rand::gen_range(0.0, TAU);
        let verticies = Self::gen_verticies(&size, edges);
        let bounding_radius = verticies.iter().map(|v| v.length()).fold(0.0_f32, f32::max);
        let spin = rand::gen_range(-PI, PI);

        Asteroid {
            size,
            position,
            rotation,
            verticies,
            bounding_radius,
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

        vec_util::convex_hull(verticies)
    }

    pub fn bounds(&self) -> (Vec2, f32) {
        (self.position, self.bounding_radius)
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

    pub fn set_position(&mut self, position: Vec2) {
        self.position = position;
        self.wrap_position();
    }

    pub fn spin(&self) -> f32 {
        self.spin
    }

    pub fn set_spin(&mut self, spin: f32) {
        self.spin = spin.clamp(-PI, PI);
    }

    pub fn gen_position(avoid_pos: Vec2, min_clearance: f32, other_asteroids: &[Asteroid]) -> Vec2 {
        loop {
            let candidate = vec2(
                rand::gen_range(0.0, layout::WORLD_W),
                rand::gen_range(0.0, layout::WORLD_H),
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
        let w = layout::WORLD_W;
        let h = layout::WORLD_H;
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
        self.position.x = self.position.x.rem_euclid(layout::WORLD_W);

        self.position.y = self.position.y.rem_euclid(layout::WORLD_H);
    }

    pub fn find_collisions(asteroids: &[Asteroid]) -> Vec<(usize, usize)> {
        let mut collisions: Vec<(usize, usize)> = Vec::new();
        for i in 0..asteroids.len() {
            let (a_i_pos, a_i_rad) = asteroids[i].bounds();
            for j in i + 1..asteroids.len() {
                let (a_j_pos, a_j_rad) = asteroids[j].bounds();

                if vec_util::circles_overlap_wrapped(a_i_pos, a_i_rad, a_j_pos, a_j_rad) {
                    collisions.push((i, j));
                }
            }
        }
        collisions
    }

    pub fn collide_with(&mut self, other: &mut Asteroid) {
        let self_pos = self.position;
        let other_pos = other.position;

        // shared frame, other ghosted next to self for wrap_position
        let a_world = self.world_vertices(self_pos);
        let b_world = other.world_vertices(self_pos);

        let Some((mut n, overlap)) = vec_util::sat_overlap(&a_world, &b_world) else {
            return;
        };

        // orient the normal from self -> other
        let dir = vec_util::wrapped_delta(self_pos, other_pos);
        if n.dot(dir) < 0.0 {
            n = -n;
        }

        let rel = self.velocity() - other.velocity();

        let (inv1, inv2) = (1.0 / self.size().mass(), 1.0 / other.size().mass());

        let along = rel.dot(n);
        if along > 0.0 {
            let e = 1.0; // restitution; 1.0 = perfectly elastic
            let j = (1.0 + e) * along / (inv1 + inv2);
            self.set_velocity(self.velocity() - ((j * inv1) * n));
            other.set_velocity(other.velocity() + ((j * inv2) * n));

            // Update spin for each asteroid
            let tangent = vec2(-n.y, n.x);
            let vt = rel.dot(tangent); // tangential relative speed

            let spin_delta = K * vt / self.size().radius();
            self.set_spin(self.spin() + spin_delta);

            let spin_delta = K * vt / other.size().radius();
            other.set_spin(other.spin() + spin_delta);
        }

        // mass-weighted de-penetration
        self.set_position(self_pos - (n * (overlap * inv1 / (inv1 + inv2))));
        other.set_position(other_pos + (n * (overlap * inv2 / (inv1 + inv2))));
    }

    pub fn contains_point(&self, p: Vec2) -> bool {
        // get point position relative to asteroid centre
        let d = vec_util::wrapped_delta(self.position, p);

        if d.length_squared() > self.bounding_radius * self.bounding_radius {
            return false;
        }

        // transform point with asteroid's rotation
        let local = vec_util::rotate(-self.rotation, d);

        vec_util::point_in_polygon(local, &self.verticies)
    }

    pub fn intersects_segment(&self, end: Vec2, step: Vec2) -> bool {
        let e_world = vec_util::wrapped_delta(self.position, end); // end, relative to center
        let s_world = e_world - step;
        let s = vec_util::rotate(-self.rotation, s_world);
        let e = vec_util::rotate(-self.rotation, e_world);

        if vec_util::point_in_polygon(s, &self.verticies)
            || vec_util::point_in_polygon(e, &self.verticies)
        {
            return true;
        }

        let n = self.verticies.len();
        for i in 0..n {
            if vec_util::segments_cross(s, e, self.verticies[i], self.verticies[(i + 1) % n]) {
                return true;
            }
        }

        false
    }

    pub fn world_vertices(&self, reference: Vec2) -> Vec<Vec2> {
        let center = reference + vec_util::wrapped_delta(reference, self.position);
        self.verticies
            .iter()
            .map(|v| center + vec_util::rotate(self.rotation, *v))
            .collect()
    }

    pub fn split(&self) -> Vec<Asteroid> {
        let Some(size) = self.size.smaller() else {
            return Vec::new();
        };

        let direction = Self::gen_direction();
        let velocity = direction * size.initial_speed();
        vec![
            Asteroid::with_velocity(size, self.position + direction * size.radius(), velocity),
            Asteroid::with_velocity(size, self.position - direction * size.radius(), -velocity),
        ]
    }
}
