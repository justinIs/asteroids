use macroquad::prelude::*;

pub fn rotate(rotation: f32, p: Vec2) -> Vec2 {
    let (sin, cos) = rotation.sin_cos();
    vec2(p.x * cos - p.y * sin, p.x * sin + p.y * cos)
}

pub fn circles_overlap_wrapped(c1: Vec2, r1: f32, c2: Vec2, r2: f32) -> bool {
    wrapped_delta(c1, c2).length_squared() < (r1 + r2).powi(2)
}

pub fn wrapped_delta(p1: Vec2, p2: Vec2) -> Vec2 {
    let mut dx = p2.x - p1.x;
    if dx > (screen_width() / 2.0) {
        dx -= screen_width();
    } else if dx < -(screen_width() / 2.0) {
        dx += screen_width();
    }

    let mut dy = p2.y - p1.y;
    if dy > (screen_height() / 2.0) {
        dy -= screen_height();
    } else if dy < -(screen_height() / 2.0) {
        dy += screen_height();
    }

    vec2(dx, dy)
}
