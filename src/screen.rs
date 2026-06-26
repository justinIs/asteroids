use crate::{game, start_screen};

pub enum Screen {
    Start(start_screen::StartScreen),
    Playing(game::Game),
    GameOver(game::Game, bool),
    Empty,
}
