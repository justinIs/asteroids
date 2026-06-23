use macroquad::prelude::*;

use crate::layout;

pub fn rotate(rotation: f32, p: Vec2) -> Vec2 {
    let (sin, cos) = rotation.sin_cos();
    vec2(p.x * cos - p.y * sin, p.x * sin + p.y * cos)
}

pub fn circles_overlap_wrapped(c1: Vec2, r1: f32, c2: Vec2, r2: f32) -> bool {
    wrapped_delta(c1, c2).length_squared() < (r1 + r2).powi(2)
}

pub fn wrapped_delta(p1: Vec2, p2: Vec2) -> Vec2 {
    let mut dx = p2.x - p1.x;
    if dx > (layout::WORLD_W / 2.0) {
        dx -= layout::WORLD_W;
    } else if dx < -(layout::WORLD_W / 2.0) {
        dx += layout::WORLD_W;
    }

    let mut dy = p2.y - p1.y;
    if dy > (layout::WORLD_H / 2.0) {
        dy -= layout::WORLD_H;
    } else if dy < -(layout::WORLD_H / 2.0) {
        dy += layout::WORLD_H;
    }

    vec2(dx, dy)
}

pub fn polygons_overlap(a: &[Vec2], b: &[Vec2]) -> bool {
    let (na, nb) = (a.len(), b.len());
    for i in 0..na {
        let (a1, a2) = (a[i], a[(i + 1) % na]);

        for j in 0..nb {
            let (b1, b2) = (b[j], b[(j + 1) % nb]);
            if segments_cross(a1, a2, b1, b2) {
                return true;
            }
        }
    }
    return point_in_polygon(a[0], b) || point_in_polygon(b[0], a);
}

fn segments_cross(a: Vec2, b: Vec2, c: Vec2, d: Vec2) -> bool {
    let d1 = (b - a).perp_dot(c - a);
    let d2 = (b - a).perp_dot(d - a);
    let d3 = (d - c).perp_dot(a - c);
    let d4 = (d - c).perp_dot(b - c);
    (d1 * d2 < 0.0) && (d3 * d4 < 0.0)
}

pub fn point_in_polygon(p: Vec2, verts: &[Vec2]) -> bool {
    let n = verts.len();
    let mut inside = false;
    let mut j = n - 1; // j trails i so (j, i) is each edge

    for i in 0..n {
        let vi = verts[i];
        let vj = verts[j];

        // check if edge straddles the horizontal line y = p.y
        if (vi.y > p.y) != (vj.y > p.y) {
            // x where the edge crosses that line (linear interpolation)
            let x_cross = (vj.x - vi.x) * (p.y - vi.y) / (vj.y - vi.y) + vi.x;
            if p.x < x_cross {
                inside = !inside;
            }
        }
        j = i;
    }
    inside
}
