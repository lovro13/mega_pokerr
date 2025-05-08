use mega_pokerr::sdl2_app::betting_state::run_betting_state;
use sdl2::image::{self, InitFlag};

use mega_pokerr::logic::game;
use mega_pokerr::logic::player;
use mega_pokerr::logic::round::begin_round;
use mega_pokerr::sdl2_app::constants::{SCREEN_HEIGHT, SCREEN_WIDTH};

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

    let player_list = player::Player::init_players();
    let game = game::init_game(player_list);

    let mut event_pump = sdl_context.event_pump().unwrap();
    // zazna inpute

    // GLAVNA ZANKA
    loop {
        {
            let mut mut_game = game.borrow_mut();
            begin_round(&mut mut_game);
        }
        
        run_betting_state(&mut canvas, &mut event_pump, &game, &font)?;
        
        break;
    }
    println!("Stopped app at the end of main sdl2_app");
    Ok(())
}
