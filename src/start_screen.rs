use crate::{
    build_info, input,
    layout::{self, WORLD_W},
    text, transition,
};
use macroquad::prelude::*;

const CURSOR_INTERVAL: f32 = 0.5;

pub struct StartScreenControls {
    menu_down: bool,
    menu_up: bool,
    select: bool,
}

impl StartScreenControls {
    pub fn from_input(i: &input::Input) -> Self {
        Self {
            menu_down: i.is_key_down(KeyCode::Down) || i.is_key_down(KeyCode::S),
            menu_up: i.is_key_down(KeyCode::Up) || i.is_key_down(KeyCode::W),
            select: is_key_pressed(KeyCode::Enter) || is_key_pressed(KeyCode::Space),
        }
    }
}

pub struct StartScreen {
    cursor_pos: usize,
    cursor_cooldown: f32,
    show_debug: bool,
}

impl StartScreen {
    pub fn new() -> Self {
        Self {
            cursor_pos: 0,
            cursor_cooldown: 0.0,
            show_debug: false,
        }
    }

    pub fn draw(&self) {
        if !self.show_debug {
            self.draw_title();
        } else {
            self.draw_debug();
        }

        self.draw_menu();
    }

    fn draw_title(&self) {
        let title = "ASTEROIDS";
        let font_size = 90.0;

        let dims = measure_text(title, None, font_size as u16, 1.0);
        let x = (layout::WORLD_W - dims.width) / 2.0;
        let y = (layout::WORLD_H - dims.height) / 2.0;

        // Draws 3D "extruded" body
        // Draw the text many times, stepping diagonally and going from dark to
        // bright. Each copy is the "side" of the letters; the last copy is the face.
        let depth = 8;
        for i in (1..=depth).rev() {
            let offset = i as f32;
            // Fade from near-black (back) to mid-grey (just behind the face)
            let shade = 0.15 + 0.25 * (1.0 - i as f32 / depth as f32);
            let color = Color::new(shade, shade, shade, 1.0);
            draw_text(title, x + offset, y + offset, font_size, color);
        }

        // Bright font face on top of the stack
        text::draw_outlined(title, x, y, font_size, WHITE, BLACK);

        // Pulsing, edge-fading underline
        let t = get_time() as f32;
        let pulse = 0.5 + 0.5 * (t * 2.5).sin(); // 0.0 .. 1.0
        let line_y = y + 12.0;
        let half = dims.width / 2.0;
        let center_x = x + half;

        let segments = 40;
        for i in 0..segments {
            // p goes 0 -> 1 across the line; map it to -1 .. 1 around the center
            let p = i as f32 / (segments - 1) as f32;
            let from_center = (p * 2.0 - 1.0).abs(); // 0 at middle, 1 at the ends
            let alpha = (1.0 - from_center) * pulse;

            let seg_x = x + p * dims.width;
            let seg_w = dims.width / segments as f32 + 1.0; // +1 to avoid gaps
            draw_rectangle(seg_x, line_y, seg_w, 3.0, Color::new(1.0, 1.0, 1.0, alpha));
        }
        let _ = center_x;
        let _ = half;

        // Credit
        let credit = "by JMAN";
        let credit_size = 24.0;
        let credit_dims = measure_text(credit, None, credit_size as u16, 1.0);
        let credit_x = (layout::WORLD_H - credit_dims.width) / 2.0;
        let credit_y = line_y + credit_dims.height + 14.0;
        text::draw_outlined(credit, credit_x, credit_y, credit_size, GRAY, BLACK);
    }

    fn draw_debug(&self) {
        let font_size = 30.0;
        let text = format!("Build Date: {}", build_info::BUILD_TIME);
        let dims = measure_text(&text, None, font_size as u16, 1.0);
        let x = (layout::WORLD_W - dims.width) / 2.0;
        let y = (layout::WORLD_H - dims.height) / 2.0;

        text::draw_outlined(&text, x, y, font_size, WHITE, BLACK);

        let text = format!("Git Hash: {}", build_info::BUILD_GIT_HASH);
        let dims = measure_text(&text, None, font_size as u16, 1.0);
        let x = (layout::WORLD_W - dims.width) / 2.0;
        let y = y + dims.height + 8.0;

        text::draw_outlined(&text, x, y, font_size, WHITE, BLACK);

        let text = format!(
            "scale: {:.2}, DPI: {}, screen: {:.0}x{:.0}",
            layout::ui_scale(),
            miniquad::window::dpi_scale(),
            screen_width(),
            screen_height()
        );
        let dims = measure_text(&text, None, font_size as u16, 1.0);
        let x = (layout::WORLD_W - dims.width) / 2.0;
        let y = y + dims.height + 8.0;

        text::draw_outlined(&text, x, y, font_size, WHITE, BLACK);
    }

    fn draw_menu(&self) {
        let font_size = 30.0;
        let font_scale = 1.0;

        let x_gap = 10.0;
        let y_gap = 20.0;
        let y_pad = 60.0;

        let caret = ">";
        let new_game_text = "New Game";
        let debug_info_text = if self.show_debug {
            "Hide Debug Info"
        } else {
            "Debug Info"
        };

        let caret_dims = measure_text(caret, None, font_size as u16, font_scale);
        let new_game_text_dims = measure_text(new_game_text, None, font_size as u16, font_scale);
        let debug_info_text_dims =
            measure_text(debug_info_text, None, font_size as u16, font_scale);

        let x = (layout::WORLD_W - new_game_text_dims.width.max(debug_info_text_dims.width)) / 2.0;
        let y = layout::WORLD_H
            - (new_game_text_dims.height + debug_info_text_dims.height + y_gap + y_pad);

        draw_text(new_game_text, x, y, font_size, WHITE);

        let mut y = layout::WORLD_W - (debug_info_text_dims.height + y_pad);

        draw_text(debug_info_text, x, y, font_size, WHITE);

        if self.cursor_pos == 0 {
            y = layout::WORLD_H
                - (new_game_text_dims.height + debug_info_text_dims.height + y_gap + y_pad);
        }

        let x = x - x_gap - caret_dims.width;

        draw_text(caret, x, y, font_size, WHITE);
    }

    pub fn update(&mut self, c: &StartScreenControls, dt: f32) -> transition::Transition {
        if self.cursor_cooldown > 0.0 {
            self.cursor_cooldown -= dt;
        } else {
            if c.menu_down && !c.menu_up {
                self.cursor_pos = (self.cursor_pos + 1).rem_euclid(2);
                self.cursor_cooldown = CURSOR_INTERVAL;
            } else if c.menu_up && !c.menu_down {
                self.cursor_pos = (self.cursor_pos - 1).rem_euclid(2);
                self.cursor_cooldown = CURSOR_INTERVAL;
            }
        }

        if c.select {
            match self.cursor_pos {
                0 => return transition::Transition::NewGame,
                1 => self.show_debug = !self.show_debug,
                _ => self.cursor_pos = 0,
            }
        }

        transition::Transition::None
    }
}
