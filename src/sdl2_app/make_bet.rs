use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::ttf::Font;
use sdl2::video::Window;
use sdl2::EventPump;
use std::time::Duration;

use crate::logic::constants::BIG_BLIND;
use crate::logic::game::Game;
use crate::logic::player::Player;
use crate::sdl2_app::render_button::Button;
use crate::sdl2_app::render_screen::render_screen;

pub fn make_bet(
    player: &Player,
    req_bet: u32,
    event_pump: &mut EventPump,
    fold_button: &mut Button,
    call_button: &mut Button,
    raise_button: &mut Button,
    canvas: &mut Canvas<Window>,
    font: &Font
) -> Option<u32> {
    loop {
        Button::draw_button(&fold_button, canvas, &font);
        Button::draw_button(&call_button, canvas, &font);
        Button::draw_button(&raise_button, canvas, &font);
        for event in event_pump.poll_iter() {
            // se sprehodi cez use evente
            Button::handle_button_events(&event, fold_button);
            Button::handle_button_events(&event, call_button);
            Button::handle_button_events(&event, raise_button);

            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    return None;
                }
                _ => {}
            }
        }
        if fold_button.is_clicked {
            return None;
        } else if call_button.is_clicked {
            if req_bet <= player.chips {
                return Some(req_bet);
            } else {
                continue;
            }
        } else if raise_button.is_clicked {
            if player.chips >= req_bet + BIG_BLIND {
                return Some(req_bet + BIG_BLIND);
            } else {
                continue;
            }
        }
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30))
    }
}
