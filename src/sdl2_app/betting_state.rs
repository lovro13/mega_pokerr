use sdl2::pixels::Color;
use sdl2::ttf::Font;
use sdl2::{render::Canvas, video::Window, EventPump};

use std::time::Duration;
use std::rc::Rc;
use std::cell::RefCell;

use crate::logic::betting_system::make_bets;
use crate::logic::game::Game;
use crate::logic::player::Player;
use crate::sdl2_app::send_bet;
use crate::sdl2_app::render_button::Button;
use crate::sdl2_app::render_screen::render_screen;

use super::constants::BACKGROUND_COLOR;

pub fn run_betting_state(
    canvas: &mut Canvas<Window>,
    event_pump: &mut EventPump,
    game: &mut Rc<RefCell<Game>>,
    font: &Font,
) -> Result<(), String> {
    // let mut mut_game = game.borrow_mut();
    let unmut_game = game.borrow();
    render_screen(canvas, Color::RGB(200, 200, 255), &unmut_game.clone(), &font)?; // nariše use kar vidiš

    let get_bet = {
        // Prenesi potrebne komponente v zaprtje
        let event_pump = event_pump; // Predpostavka: event_pump je `&mut EventPump`
         // Če `Game` implementira `Clone`
    
        move |player: &Player, req_bet: u32| -> Option<u32> {
            // 1. Ustvari NOVE gumbe za vsakega igralca
            let mut fold_button = Button::init_fold_button(canvas);
            let mut call_button = Button::init_call_button(canvas);
            let mut raise_button = Button::init_raise_button(canvas);
    
            // 2. Počisti event_pump pred novo iteracijo
            let _: Vec<_> = event_pump.poll_iter().collect();
    
            // 3. Render s kloniranim stanjem (če je potrebno)
            render_screen(
                canvas,
                Color::RGB(BACKGROUND_COLOR.0, BACKGROUND_COLOR.1, BACKGROUND_COLOR.2),
                &unmut_game, // Uporabi klonirano stanje
                &font,
            ).unwrap();
    
            // 4. Pokliči make_bet s svežimi gumbi in očiščenim event_pump
            send_bet::make_bet(
                player,
                req_bet,
                event_pump, // Preveri, ali je event_pump pravilno uporabljen
                &mut fold_button,
                &mut call_button,
                &mut raise_button,
                canvas,
                &font,
            )
            .unwrap()
        }
    };
    
    make_bets(&mut Rc::clone(game), get_bet);
    ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
    Ok(())
}
