use sdl2::ttf::Font;
use sdl2::{render::Canvas, video::Window, EventPump};

use std::cell::RefCell;
use std::rc::Rc;
use std::sync::atomic::Ordering;

use crate::logic::betting_system::make_bets;
use crate::logic::constants::SHOULD_QUIT;
use crate::logic::game::Game;
use crate::logic::player::Id;
use crate::sdl2_app::send_bet;

pub fn run_betting_state(
    canvas: &mut Canvas<Window>,
    event_pump: &mut EventPump,
    game: &Rc<RefCell<Game>>,
    font: &Font
) -> Result<(), String> {
    // Kloniraj Rc<RefCell<Game>> za uporabo v zaprtju

    let canvas_rc = Rc::new(RefCell::new(canvas));
    // Definiraj get_bet
    let get_bet = {
        // Prenesi reference na canvas in font
        let event_pump = event_pump; // &mut EventPump
        let font = font;

        move |game: &Game, req_bet: u32| -> Option<u32> {
            let player = game.player_on_turn_immutable();
            let canvas = Rc::clone(&canvas_rc);
            let mut canvas_borrow = canvas.borrow_mut();
            if player.id == Id::Player1 {
                send_bet::make_bet_player1(
                    player,
                    req_bet,
                    event_pump,
                    &mut *canvas_borrow,
                    font,
                    game
                )
                .unwrap()
            } else {
                send_bet::make_bet_bot(player, req_bet, event_pump, &mut *canvas_borrow, font, game).unwrap()
            }
        }
    };

    // Ločen scope za mutabilen dostop v make_bets
    {
        let mut game_mut  = game.borrow_mut();
        make_bets(&mut *game_mut, get_bet);
        if SHOULD_QUIT.load(Ordering::Relaxed) {
            return Ok(());
        }
    }

    Ok(())
}
