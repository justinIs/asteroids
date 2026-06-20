use crate::game;

pub enum Screen {
    Start,
    Playing(game::Game),
    GameOver(game::Game),
}
