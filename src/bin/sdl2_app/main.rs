use mega_pokerr::logic::betting_system::make_bets;
use mega_pokerr::sdl2_app::send_bet;
use sdl2::event::Event;
use sdl2::image::{self, InitFlag};
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::Duration;
use std::rc::Rc;

use mega_pokerr::logic::game;
use mega_pokerr::logic::player::{self, Player};
use mega_pokerr::logic::round;
use mega_pokerr::sdl2_app::constants::{SCREEN_HEIGHT, SCREEN_WIDTH};
use mega_pokerr::sdl2_app::render_button::Button;
use mega_pokerr::sdl2_app::render_screen::render_screen;

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
    let mut call_button = Button::init_call_button(&mut canvas);
    let mut raise_button = Button::init_raise_button(&mut canvas);

    canvas.clear();
    canvas.present();
    let player_list = player::Player::init_players();
    let game = game::init_game(player_list);
    let game_for_drawing = game.borrow();
    let mut mut_game = game.borrow_mut();

    let mut event_pump = sdl_context.event_pump().unwrap();
    // zazna inpute

    // GLAVNA ZANKA
    'running: loop { // ta main loop mi ni ušeč, bolje da imamo več funkicji v kateri je vsaki tak isti loopg
        for event in event_pump.poll_iter() {
            // se sprehodi cez use evente
            Button::handle_button_events(&event, &mut fold_button);
            Button::handle_button_events(&event, &mut call_button);
            Button::handle_button_events(&event, &mut raise_button);

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
                } => {
                    round::begin_round(&mut mut_game);
                }
                _ => {}
            }
        }
        // skupi z for po eventih, ki bojo pač vrnil Option<Int>
        // in tudi z render screen, zato more biti zaprtje, mogoče še tukaj definirano, da lahko dela z canvas in font
        // torej lahko tudi button narišemo v get_bets
        render_screen(
            &mut canvas,
            Color::RGB(200, 200, 255),
            &game_for_drawing,
            &font
        )?; // nariše use kar vidiš
        
        
        let get_bet = |player: &Player, req_bet: u32| -> Option<u32> {
            send_bet::make_bet(
                player,
                req_bet,
                &mut event_pump,
                &mut fold_button,
                &mut call_button,
                &mut raise_button,
                &mut canvas,
                &font
            ).unwrap()
        };
        make_bets(&mut Rc::clone(&game), get_bet);
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30))
        // nastavi na cca 30 FPS
    }

    Ok(())
}
