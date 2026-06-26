use macroquad::prelude::*;

use crate::{
    game::{Game, GameControls},
    screen::Screen,
    transition::Transition,
};

mod build_info;
mod camera;
mod game;
mod input;
mod layout;
mod screen;
mod start_screen;
mod text;
mod transition;
mod vec_util;

fn window_conf() -> Conf {
    Conf {
        window_title: "Asteroids".to_owned(),
        window_width: 900,
        window_height: 900,
        window_resizable: false,
        high_dpi: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    println!("screen: {}, {}", screen_width(), screen_height());

    let mut screen = Screen::Start(start_screen::StartScreen::new());

    let mut i = input::Input::new();

    loop {
        clear_background(BLACK);

        i.update();

        set_camera(&camera::world_camera());

        let dt = get_frame_time();

        let transition = match &mut screen {
            Screen::Start(start_screen) => {
                start_screen.update(&start_screen::StartScreenControls::from_input(&i), dt)
            }
            Screen::Playing(game) => game.update(dt, &GameControls::from_input(&i)),
            Screen::GameOver(_, _) => {
                if i.any_press() {
                    Transition::ToStart
                } else {
                    Transition::None
                }
            }
            Screen::Empty => Transition::None,
        };
        // Consume inputs held down between transitions
        if !matches!(transition, Transition::None) {
            i.consume();
        }

        match &screen {
            Screen::Start(start_screen) => start_screen.draw(),
            Screen::Playing(game) => {
                game.draw();
                // draw_debug_info(i.pointers.len());
            }
            Screen::GameOver(game, win) => {
                game.draw();
                if *win {
                } else {
                    draw_game_over(game.score())
                }
            }
            Screen::Empty => (),
        }

        set_default_camera();
        if i.using_touch && matches!(screen, Screen::Playing(_)) {
            game::draw_touch_buttons(&i);
        }

        match transition {
            Transition::None => {}
            Transition::NewGame => screen = Screen::Playing(Game::new()),
            Transition::ToStart => screen = Screen::Start(start_screen::StartScreen::new()),
            Transition::GameOver(win) => {
                let old = std::mem::replace(&mut screen, Screen::Empty);
                if let Screen::Playing(game) = old {
                    screen = Screen::GameOver(game, win);
                }
            }
        }

        next_frame().await;
    }
}

fn draw_game_over(score: u32) {
    let font_size = 80.0;

    let game_over_text = "GAME OVER";
    let game_over_dims = measure_text(game_over_text, None, font_size as u16, 1.0);
    let game_over_x = (layout::WORLD_W - game_over_dims.width) / 2.0;
    let game_over_y = (layout::WORLD_H - game_over_dims.height) / 2.0;

    text::draw_outlined(
        game_over_text,
        game_over_x,
        game_over_y,
        font_size,
        WHITE,
        BLACK,
    );

    let score_text = format!("SCORE: {}", score);
    let dims = measure_text(&score_text, None, 60.0 as u16, 1.0);
    let x = (layout::WORLD_W - dims.width) / 2.0;
    let y = game_over_y + game_over_dims.height + 8.0;

    text::draw_outlined(&score_text, x, y, 60.0, WHITE, BLACK);
}
