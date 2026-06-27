use crate::{build_info, input, layout, text, transition};
use macroquad::prelude::*;

const CURSOR_INTERVAL: f32 = 0.5;

pub struct StartScreenControls {
    menu_down: bool,
    menu_up: bool,
    select: bool,
    // touch controls
    new_game: bool,
    toggle_debug_info: bool,
}

impl StartScreenControls {
    pub fn from_input(i: &input::Input, is_debug_showing: bool) -> Self {
        let mut c = Self {
            menu_down: i.is_key_down(KeyCode::Down) || i.is_key_down(KeyCode::S),
            menu_up: i.is_key_down(KeyCode::Up) || i.is_key_down(KeyCode::W),
            select: is_key_pressed(KeyCode::Enter) || is_key_pressed(KeyCode::Space),
            new_game: false,
            toggle_debug_info: false,
        };

        if i.using_touch {
            let menu = menu_buttons(is_debug_showing);
            for b in menu.buttons {
                if i.is_pressed_world(b.rect) {
                    match b.action {
                        Action::NewGame => c.new_game = true,
                        Action::ToggleDebug => c.toggle_debug_info = true,
                    }
                }
            }
        }

        c
    }
}

pub struct StartScreen {
    cursor_pos: usize,
    cursor_cooldown: f32,
    show_debug: bool,
    using_touch: bool,
}

impl StartScreen {
    pub fn new() -> Self {
        Self {
            cursor_pos: 0,
            cursor_cooldown: 0.0,
            show_debug: false,
            using_touch: false,
        }
    }

    pub fn draw(&self) {
        if !self.show_debug {
            self.draw_title();
        } else {
            self.draw_debug();
        }

        let menu = menu_buttons(self.show_debug);
        for (i, b) in menu.buttons.iter().enumerate() {
            draw_rectangle_lines(
                b.rect.x,
                b.rect.y,
                b.rect.w,
                b.rect.h,
                2.0 * layout::ui_scale(),
                WHITE,
            );
            let x = b.rect.x + menu.padding + menu.caret_w + menu.x_gap;
            let y = b.rect.y + b.rect.h / 2.0 + menu.padding;
            draw_text(b.label, x, y, menu.font_size, WHITE);
            if !self.using_touch && i == self.cursor_pos {
                draw_text(
                    menu.caret,
                    x - menu.caret_w - menu.x_gap,
                    y,
                    menu.font_size,
                    WHITE,
                );
            }
        }
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

        // Credit
        let credit = "by JMAN";
        let credit_size = 24.0;
        let credit_dims = measure_text(credit, None, credit_size as u16, 1.0);
        let credit_x = (layout::WORLD_W - credit_dims.width) / 2.0;
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

    pub fn update(&mut self, i: &mut input::Input, dt: f32) -> transition::Transition {
        let c = StartScreenControls::from_input(i, self.show_debug);
        if i.using_touch {
            self.using_touch = i.using_touch;
            if c.new_game {
                return transition::Transition::NewGame;
            }
            if c.toggle_debug_info {
                self.show_debug = !self.show_debug;
                i.consume();
            }
        }
        if self.cursor_cooldown > 0.0 {
            self.cursor_cooldown -= dt;
        } else {
            if c.menu_down && !c.menu_up {
                let n = 2;
                self.cursor_pos = (self.cursor_pos + 1) % n;
                self.cursor_cooldown = CURSOR_INTERVAL;
            } else if c.menu_up && !c.menu_down {
                let n = 2;
                self.cursor_pos = (self.cursor_pos + n - 1) % n;
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

#[derive(Clone, Copy)]
enum Action {
    NewGame,
    ToggleDebug,
}

struct Button {
    rect: Rect,
    label: &'static str,
    action: Action,
}

struct MenuUI {
    buttons: [Button; 2],
    padding: f32,
    caret: &'static str,
    font_size: f32,
    caret_w: f32,
    x_gap: f32,
}

fn menu_buttons(is_debug_showing: bool) -> MenuUI {
    let font_size = 30.0;
    let font_scale = 1.0;

    let x_gap = 10.0;
    let y_gap = 20.0;
    let button_padding = 10.0;
    let y_bottom_margin = 60.0;

    let caret = ">";
    let new_game_text = "New Game";
    let debug_info_text = if is_debug_showing {
        "Hide Debug Info"
    } else {
        "Debug Info"
    };

    let caret_dims = measure_text(caret, None, font_size as u16, font_scale);
    let new_game_text_dims = measure_text(new_game_text, None, font_size as u16, font_scale);
    let debug_info_text_dims = measure_text(debug_info_text, None, font_size as u16, font_scale);

    // Text Alignment
    let x = (layout::WORLD_W - new_game_text_dims.width.max(debug_info_text_dims.width)) / 2.0;

    let y = layout::WORLD_H
        - (new_game_text_dims.height
            + debug_info_text_dims.height
            + y_gap
            + 2.0 * button_padding
            + y_bottom_margin);
    let button_x = x - button_padding - caret_dims.width - x_gap;
    let w = new_game_text_dims.width.max(debug_info_text_dims.width)
        + caret_dims.width
        + x_gap
        + (2.0 * button_padding);
    let h = new_game_text_dims.height.max(debug_info_text_dims.height) + (2.0 * button_padding);

    MenuUI {
        buttons: [
            Button {
                rect: Rect::new(button_x, y - button_padding, w, h),
                label: new_game_text,
                action: Action::NewGame,
            },
            Button {
                rect: Rect::new(
                    button_x,
                    y + new_game_text_dims.height + button_padding + y_gap,
                    w,
                    h,
                ),
                label: debug_info_text,
                action: Action::ToggleDebug,
            },
        ],
        font_size,
        caret,
        x_gap,
        padding: button_padding,
        caret_w: caret_dims.width,
    }
}
