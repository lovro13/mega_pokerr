use sdl2::keyboard::Keycode;
use sdl2::render::Canvas;
use sdl2::ttf::Font;
use sdl2::video::Window;
use sdl2::EventPump;
use sdl2::{event::Event, pixels::Color};
use std::time::Duration;

use crate::logic::constants::BIG_BLIND;
use crate::logic::game::Game;
use crate::logic::player::Player;
use crate::sdl2_app::render_button::Button;

use super::render_screen::{render_screen, render_turn_indicator};

pub fn make_bet(
    player: &Player,
    req_bet: u32,
    event_pump: &mut EventPump,
    fold_button: &mut Button,
    call_button: &mut Button,
    raise_button: &mut Button,
    canvas: &mut Canvas<Window>,
    font: &Font,
    game: &Game,
) -> Result<Option<u32>, String> {
    let _: Vec<_> = event_pump.poll_iter().collect();
    loop {
        for event in event_pump.poll_iter() {
            // se sprehodi cez use evente
            Button::handle_button_events(&event, fold_button);
            Button::handle_button_events(&event, call_button);
            Button::handle_button_events(&event, raise_button);
            canvas.present();
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    panic!("ustavljam igro z panic, ker drugače ne gre :) <3");
                }
                _ => {}
            }
        }
        if fold_button.is_clicked {
            println!("folding in sld2app");
            return Ok(None);
        } else if call_button.is_clicked {
            if req_bet <= player.chips {
                return Ok(Some(req_bet));
            } else {
                continue;
            }
        } else if raise_button.is_clicked {
            if player.chips >= req_bet + BIG_BLIND {
                return Ok(Some(req_bet + BIG_BLIND));
            } else {
                continue;
            }
        }
        let (r, g, b) = (173, 216, 230); // Light blue color
        render_screen(canvas, Color::RGB(r, g, b), game, font)?;
        Button::draw_button(&fold_button, canvas, &font)?;
        Button::draw_button(&call_button, canvas, &font)?;
        Button::draw_button(&raise_button, canvas, &font)?;
        render_turn_indicator(player, canvas)?;
        canvas.present();
        if fold_button.is_clicked || call_button.is_clicked || raise_button.is_clicked {
            ::std::thread::sleep(Duration::from_millis(200));
        }
        ::std::thread::sleep(Duration::from_millis(33));
    }
}
