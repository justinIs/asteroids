use macroquad::prelude::*;

const ROTATION_SPEED: f32 = std::f32::consts::PI;
const THRUST: f32 = 200.0;
const MAX_VELOCITY: f32 = 400.0;

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

    pub fn update(&mut self, dt: f32) {
        // Handle rotation
        let right = is_key_down(KeyCode::Right) || is_key_down(KeyCode::D);
        let left = is_key_down(KeyCode::Left) || is_key_down(KeyCode::A);
        if right && !left {
            self.rotation += ROTATION_SPEED * dt;
        } else if left && !right {
            self.rotation -= ROTATION_SPEED * dt;
        }
        self.rotation = self.rotation.rem_euclid(std::f32::consts::TAU);

        // Handle thrust
        let up = is_key_down(KeyCode::Up) || is_key_down(KeyCode::W);
        if up {
            let direction = vec2(self.rotation.sin(), -self.rotation.cos());
            self.velocity += direction * THRUST * dt;
            self.velocity = self.velocity.clamp_length_max(MAX_VELOCITY);
        }

        // Update position based on velocity
        self.position += self.velocity * dt;
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
    }

    pub fn draw(&self) {
        let nose = vec2(0.0, -18.0);
        let left_rear = vec2(-12.0, 12.0);
        let right_rear = vec2(12.0, 12.0);
        let back = vec2(0.0, 5.0);

        let [n, l, r, b] =
            [nose, left_rear, right_rear, back].map(|p| self.rotate(p) + self.position);

        draw_line(l.x, l.y, n.x, n.y, 1.5, WHITE);
        draw_line(n.x, n.y, r.x, r.y, 1.5, WHITE);
        draw_line(r.x, r.y, b.x, b.y, 1.5, WHITE);
        draw_line(b.x, b.y, l.x, l.y, 1.5, WHITE);
    }

    fn rotate(&self, p: Vec2) -> Vec2 {
        let (sin, cos) = self.rotation.sin_cos();
        vec2(p.x * cos - p.y * sin, p.x * sin + p.y * cos)
    }
}
