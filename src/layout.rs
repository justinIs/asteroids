pub const WORLD_W: f32 = 600.0;
pub const WORLD_H: f32 = 600.0;

pub fn ui_scale() -> f32 {
    #[cfg(target_os = "android")]
    {
        // Derive multiplier from short screen side, using 600 as the "design" short side - lower to
        // make buttons bigger
        (macroquad::window::screen_width().min(macroquad::window::screen_height()) / 400.0).max(1.0)
    }
    #[cfg(not(target_os = "android"))]
    {
        1.0
    }
}
