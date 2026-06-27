use macroquad::prelude::*;

use crate::layout;

pub fn world_camera() -> Camera2D {
    let scale = (screen_width() / layout::WORLD_W).min(screen_height() / layout::WORLD_H);
    let (vw, vh) = (layout::WORLD_W * scale, layout::WORLD_H * scale);

    let mut cam =
        Camera2D::from_display_rect(Rect::new(0.0, 0.0, layout::WORLD_W, layout::WORLD_H));
    cam.zoom.y = -cam.zoom.y;
    let dpi = miniquad::window::dpi_scale();
    cam.viewport = Some((
        ((screen_width() - vw) / 2.0 * dpi) as i32, // center horizontally
        ((screen_height() - vh) / 2.0 * dpi) as i32, // center vertically
        (vw * dpi) as i32,
        (vh * dpi) as i32,
    ));

    cam
}

pub fn screen_to_world(p: Vec2) -> Vec2 {
    let scale = (screen_width() / layout::WORLD_W).min(screen_height() / layout::WORLD_H);
    let vw = layout::WORLD_W * scale;
    let vh = layout::WORLD_H * scale;
    let offset = vec2((screen_width() - vw) / 2.0, (screen_height() - vh) / 2.0);
    (p - offset) / scale
}
