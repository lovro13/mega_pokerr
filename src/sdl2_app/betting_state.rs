use sdl2::pixels::Color;
use sdl2::ttf::Font;
use sdl2::{render::Canvas, video::Window, EventPump};

use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;

use crate::logic::betting_system::make_bets;
use crate::logic::game::Game;
use crate::logic::player::Player;
use crate::sdl2_app::render_button::Button;
use crate::sdl2_app::render_screen::render_screen;
use crate::sdl2_app::send_bet;

use super::constants::BACKGROUND_COLOR;

pub fn run_betting_state(
    canvas: &mut Canvas<Window>,
    event_pump: &mut EventPump,
    game: &Rc<RefCell<Game>>,
    font: &Font,
) -> Result<(), String> {
    // Kloniraj Rc<RefCell<Game>> za uporabo v zaprtju
    let game_clone = Rc::clone(game);
    
    // Definiraj get_bet
    let get_bet = {
        // Prenesi reference na canvas in font
        let event_pump = event_pump; // &mut EventPump

        move |player: &Player, req_bet: u32| -> Option<u32> {
            // Ustvari gumbe
            let mut fold_button = Button::init_fold_button(canvas);
            let mut call_button = Button::init_call_button(canvas);
            let mut raise_button = Button::init_raise_button(canvas);

            // Počisti event_pump
            let _: Vec<_> = event_pump.poll_iter().collect();

            // Render s ne-mutabilnim dostopom
            {
                let game_ref = game_clone.borrow();  // Dobimo &RefCell<Game>
                render_screen(
                    canvas,
                    Color::RGB(BACKGROUND_COLOR.0, BACKGROUND_COLOR.1, BACKGROUND_COLOR.2),
                    &*game_ref,  // Dereferenciramo v &Game
                    font,
                )
                .unwrap();
            } // game_ref se sprosti tukaj

            send_bet::make_bet(
                player,
                req_bet,
                event_pump,
                &mut fold_button,
                &mut call_button,
                &mut raise_button,
                canvas,
                font,
            )
            .unwrap()
        }
    };

    // Ločen scope za mutabilen dostop v make_bets
    {
        let mut game_mut = game.borrow_mut();
        make_bets(&mut game_mut, get_bet);
    }

    Ok(())
}