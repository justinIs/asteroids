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
    point_in_polygon(a[0], b) || point_in_polygon(b[0], a)
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

/// Convex hull of a st of points (Andrew's monotone chain), returned in CCW order.
/// Sort the points, then walk the bottom edge keeping only left turns (any right
/// turn means the previous vertex was a dent — pop it), then walk the top edge
/// the same way. The cross product's sign tells you the turn direction; popping
/// on <= 0.0 also discards redundant collinear points. The result is the tightest
/// convex polygon enclosing all your points.
pub fn convex_hull(mut points: Vec<Vec2>) -> Vec<Vec2> {
    let n = points.len();
    if n < 3 {
        return points; // a hull needs at least 3 points
    }

    // sort left-to-right, braking ties bottom-to-top
    points.sort_by(|a, b| {
        a.x.partial_cmp(&b.x)
            .unwrap()
            .then(a.y.partial_cmp(&b.y).unwrap())
    });

    // (a-o) x (b-o): > 0 left turn, < 0 right turn, 0 collinear
    let cross = |o: Vec2, a: Vec2, b: Vec2| (a - o).perp_dot(b - o);

    let mut hull: Vec<Vec2> = Vec::with_capacity(2 * n);

    // lower hull: sweep left -> right, kep only left turns
    for &p in &points {
        while hull.len() >= 2 && cross(hull[hull.len() - 2], hull[hull.len() - 1], p) <= 0.0 {
            hull.pop();
        }
        hull.push(p);
    }

    // upper hull: sweep right -> left
    let lower = hull.len() + 1;
    for &p in points.iter().rev() {
        while hull.len() >= lower && cross(hull[hull.len() - 2], hull[hull.len() - 1], p) <= 0.0 {
            hull.pop();
        }
        hull.push(p);
    }

    hull.pop(); // last point duplicates first
    hull
}

/// Projects a polygon onto an axis (returns the polygon's "shadow" [min, max] along a normalized
/// axis)
pub fn project(verts: &[Vec2], axis: Vec2) -> (f32, f32) {
    let mut min = f32::INFINITY;
    let mut max = f32::NEG_INFINITY;

    for v in verts {
        let d = v.dot(axis);
        min = min.min(d);
        max = max.max(d);
    }
    (min, max)
}

pub fn sat_overlap(a: &[Vec2], b: &[Vec2]) -> Option<(Vec2, f32)> {
    let mut best_overlap = f32::INFINITY;
    let mut best_axis = Vec2::ZERO;

    for poly in [a, b] {
        let n = poly.len();
        for i in 0..n {
            // outward normal of this edge, normalized
            let edge = poly[(i + 1) % n] - poly[i];
            let axis = vec2(-edge.y, edge.x).normalize();

            // project both polygons onto axis
            let (amin, amax) = project(a, axis);
            let (bmin, bmax) = project(b, axis);
            let overlap = amax.min(bmax) - amin.max(bmin); // interval intersection

            if overlap <= 0.0 {
                return None; // a gap on this axis -> polygons are separated
            }
            if overlap < best_overlap {
                best_overlap = overlap;
                best_axis = axis;
            }
        }
    }

    Some((best_axis, best_overlap))
}
