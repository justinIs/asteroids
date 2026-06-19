use crate::game;

pub enum Screen {
    Start,
    Playing(game::Game),
    GameOver { score: u32 },
}
