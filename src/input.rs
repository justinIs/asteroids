use macroquad::prelude::*;

pub struct Input {
    pub pointers: Vec<Vec2>,
    pub using_touch: bool,
    swallow: bool, // ignore held inputs until everything is released
}

impl Input {
    pub fn new() -> Self {
        Self {
            pointers: Vec::new(),
            using_touch: false,
            swallow: false,
        }
    }
    pub fn update(&mut self) {
        self.pointers = pointers();
        if !touches().is_empty() {
            self.using_touch = true;
        }

        if self.swallow && !is_anything_active() {
            self.swallow = false
        }
    }

    pub fn consume(&mut self) {
        self.swallow = true;
    }

    pub fn any_press(&self) -> bool {
        if self.swallow {
            return false;
        }
        get_last_key_pressed().is_some() || !touches().is_empty()
    }

    pub fn is_pressed(&self, rect: Rect) -> bool {
        !self.swallow && self.pointers.iter().any(|p| rect.contains(*p))
    }

    pub fn is_pressed_world(&self, rect: Rect) -> bool {
        !self.swallow
            && self
                .pointers
                .iter()
                .any(|p| rect.contains(crate::camera::screen_to_world(*p)))
    }

    pub fn is_key_down(&self, key_code: KeyCode) -> bool {
        !self.swallow && is_key_down(key_code)
    }
}

fn is_anything_active() -> bool {
    !get_keys_down().is_empty() || !touches().is_empty() || is_mouse_button_down(MouseButton::Left)
}

fn pointers() -> Vec<Vec2> {
    let scale = miniquad::window::dpi_scale();
    let mut ps: Vec<Vec2> = touches().iter().map(|t| t.position / scale).collect();

    if is_mouse_button_down(MouseButton::Left) {
        ps.push(mouse_position().into());
    }

    ps
}
