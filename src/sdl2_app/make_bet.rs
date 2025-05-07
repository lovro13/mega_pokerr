use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::Canvas;
use sdl2::ttf::Font;
use sdl2::video::Window;
use sdl2::EventPump;
use std::time::Duration;

use crate::logic::constants::BIG_BLIND;
use crate::logic::player::Player;
use crate::sdl2_app::render_button::Button;

pub fn make_bet(
    player: &Player,
    req_bet: u32,
    event_pump: &mut EventPump,
    fold_button: &mut Button,
    call_button: &mut Button,
    raise_button: &mut Button,
    canvas: &mut Canvas<Window>,
    font: &Font
) -> Result<Option<u32>, String> {
    loop {
        Button::draw_button(&fold_button, canvas, &font)?;
        Button::draw_button(&call_button, canvas, &font)?;
        Button::draw_button(&raise_button, canvas, &font)?;
        for event in event_pump.poll_iter() {
            // se sprehodi cez use evente
            Button::handle_button_events(&event, fold_button);
            Button::handle_button_events(&event, call_button);
            Button::handle_button_events(&event, raise_button);
            let turn_indicator_color = Color::RGB(255, 0, 0);
            let turn_indicator_position = Point::new(player.card_position.0, -player.card_position.1) 
                + Point::new(50, -50); // Adjust position relative to card_position
            let turn_indicator_rect = Rect::from_center(turn_indicator_position, 20, 20);
            canvas.set_draw_color(turn_indicator_color);
            canvas.fill_rect(turn_indicator_rect)?;
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    return Ok(None);
                }
                _ => {}
            }
        }
        if fold_button.is_clicked {
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
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30))
    }
}
