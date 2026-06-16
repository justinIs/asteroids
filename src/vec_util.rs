use macroquad::prelude::*;

pub fn rotate(rotation: f32, p: Vec2) -> Vec2 {
    let (sin, cos) = rotation.sin_cos();
    vec2(p.x * cos - p.y * sin, p.x * sin + p.y * cos)
}
