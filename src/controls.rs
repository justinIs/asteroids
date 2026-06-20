use macroquad::prelude::*;

use crate::ship;

pub struct Controls {
    pub ship_controls: ship::ShipControls,
    pub fire: bool,
    pub pause: bool,
    pub using_touch: bool,
}

impl Controls {
    pub fn any_press(&self) -> bool {
        get_last_key_pressed().is_some() || !touches().is_empty()
    }
}

#[derive(Clone, Copy)]
enum Action {
    RotateLeft,
    RotateRight,
    Thrust,
    Fire,
}

struct Button {
    rect: Rect,
    label: &'static str,
    action: Action,
}

fn button_layout() -> [Button; 4] {
    let (w, h) = (screen_width(), screen_height());
    let s = 90.0; // button size
    let m = 28.0; // margin
    let y = h - s - m;
    [
        Button {
            rect: Rect::new(m, y, s, s),
            label: "<",
            action: Action::RotateLeft,
        },
        Button {
            rect: Rect::new(m + s + 16.0, y, s, s),
            label: ">",
            action: Action::RotateRight,
        },
        Button {
            rect: Rect::new(w - s - m, y - s - m, s, s),
            label: "^",
            action: Action::Thrust,
        },
        Button {
            rect: Rect::new(w - s - m, y, s, s),
            label: "0",
            action: Action::Fire,
        },
    ]
}

fn pointers() -> Vec<Vec2> {
    let mut ps: Vec<Vec2> = touches().iter().map(|t| t.position).collect();

    if is_mouse_button_down(MouseButton::Left) {
        ps.push(mouse_position().into());
    }

    ps
}

pub fn read_controls() -> Controls {
    let ps = pointers();
    let mut c = Controls {
        ship_controls: ship::ShipControls {
            rotate_left: is_key_down(KeyCode::Left) || is_key_down(KeyCode::A),
            rotate_right: is_key_down(KeyCode::Right) || is_key_down(KeyCode::D),
            thrust: is_key_down(KeyCode::Up) || is_key_down(KeyCode::W),
        },
        pause: is_key_pressed(KeyCode::Enter),
        fire: is_key_pressed(KeyCode::Space),
        using_touch: !touches().is_empty(),
    };

    for b in button_layout() {
        if ps.iter().any(|p| b.rect.contains(*p)) {
            match b.action {
                Action::RotateLeft => c.ship_controls.rotate_left = true,
                Action::RotateRight => c.ship_controls.rotate_right = true,
                Action::Thrust => c.ship_controls.thrust = true,
                Action::Fire => c.fire = true,
            }
        }
    }
    c
}

pub fn draw_touch_buttons() {
    let ps = pointers();
    for b in button_layout() {
        let pressed = ps.iter().any(|p| b.rect.contains(*p));
        let fill = if pressed {
            Color::new(1.0, 1.0, 1.0, 0.35)
        } else {
            Color::new(1.0, 1.0, 1.0, 0.12)
        };

        draw_rectangle(b.rect.x, b.rect.y, b.rect.w, b.rect.h, fill);
        draw_rectangle_lines(b.rect.x, b.rect.y, b.rect.w, b.rect.h, 2.0, WHITE);
        draw_text(
            b.label,
            b.rect.x + b.rect.w / 2.0 - 8.0,
            b.rect.y + b.rect.h / 2.0 + 8.0,
            36.0,
            WHITE,
        );
    }
}
