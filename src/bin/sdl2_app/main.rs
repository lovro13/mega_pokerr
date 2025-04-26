use sdl2::event::Event;
use sdl2::image::{self, InitFlag};
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::Duration;

use projektna_prog_2::logic::player;
use projektna_prog_2::logic::round;

use projektna_prog_2::sdl2_app::render_screen::render_screen;
use projektna_prog_2::sdl2_app::render_button::Button;
use projektna_prog_2::sdl2_app::constants::{SCREEN_WIDTH, SCREEN_HEIGHT};

pub enum GameState {
    Paused,
    Played(player::Player),
}



fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let _image_context = image::init(InitFlag::PNG | InitFlag::JPG)?;
    let window = video_subsystem
        .window("POKEEEER", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .expect("could not initialize video subsystem");

    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;
    let mut font = ttf_context
        .load_font("font/Poppins-Black.ttf", 120)
        .unwrap();
    font.set_style(sdl2::ttf::FontStyle::NORMAL);



    let mut canvas = window.into_canvas().build().expect("could not make canvas");
    // dobiš platno !! POMEMBNO, canvas.set_color(); canvas.clear() - zaslon v eno bravo
    // canvas.copy(...), texture -> riše slike, ali tekst
    // canvas.present() ... predstavi spremembe, ki so jih nardil .copy(), .clear()

    let mut fold_button = Button::init_fold_button(&mut canvas);
    let player_list = player::Player::init_players(); 

    canvas.clear();
    canvas.present();
    let mut game  = round::init_game(player_list);
    
    let mut event_pump = sdl_context.event_pump().unwrap();
    // zazna inpute

    // GLAVNA ZANKA
    'running: loop {
        for event in event_pump.poll_iter() { // se sprehodi cez use evente
            Button::handle_button_events(&event, &mut fold_button);

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
                } => {round::begin_round(&mut game);},
                _ => {}
            }
        }

        render_screen(
            &mut canvas,
            Color::RGB(200, 200, 255),
            &game,
            &font,
        )?; // nariše use kar vidiš
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30))
        // nastavi na cca 30 FPS
    }

    Ok(())
}
