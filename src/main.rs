use sdl2::event::Event;
use sdl2::image::{self, InitFlag};
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;
use std::time::Duration;

mod card;
mod player;
mod render;
mod round;

const SCREEN_HEIGHT: u32 = 900;
const SCREEN_WIDTH: u32 = 1800;

pub enum GameState {
    Paused,
    Played(player::Player),
}

fn render(
    canvas: &mut WindowCanvas,
    background_color: Color,
    players_list: &Vec<player::Player>,
    font: &sdl2::ttf::Font,
) -> Result<(), String> {
    canvas.set_draw_color(background_color);
    canvas.clear();

    for player in players_list {
        //naprinta ime in karte igralca
        // let _ = player::Player::render_player_info(canvas, player, font);
        let _ = render::render_player_info(canvas, player, font);
    }
    canvas.present();
    Ok(())
}

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;

    let video_subsystem = sdl_context.video()?;

    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;
    let mut font = ttf_context
        .load_font("font/Poppins-Black.ttf", 120)
        .unwrap();
    font.set_style(sdl2::ttf::FontStyle::NORMAL);

    let _image_context = image::init(InitFlag::PNG | InitFlag::JPG)?;

    let window = video_subsystem
        .window("POKEEEER", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .expect("could not initialize video subsystem");

    let mut canvas = window.into_canvas().build().expect("could not make canvas");

    // let position = Point::new(PLAYER1_CARDS.0, PLAYER1_CARDS.1);
    let mut player_list = player::Player::init_players();
    let mut event_pump = sdl_context.event_pump().unwrap();

    canvas.clear();
    canvas.present();
    round::begin_round(&mut player_list);

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    break 'running;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::D),
                    ..
                } => round::begin_round(&mut player_list),
                _ => {}
            }
        }

        render(&mut canvas, Color::RGB(200, 200, 255), &player_list, &font)?;
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30))
    }

    Ok(())
}
