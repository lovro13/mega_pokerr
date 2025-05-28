use sdl2::keyboard::Keycode;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::EventPump;
use sdl2::{event::Event, pixels::Color};
use std::sync::atomic::Ordering;
use std::time::Duration;

use crate::logic::constants::SHOULD_QUIT;
use crate::logic::game::Game;
use crate::logic::player::Player;
use crate::sdl2_app::render_button::Button;
use crate::sdl2_app::render_text::write_info;


use super::constants::WRITE_INFO_SIZE;
use super::positions::ControlPosition;
use super::render_screen::render_screen;
use super::slider::Slider;
use super::tactic1::make_decision;

const BUTTON_TEXT_SIZE: u16 = 30;



pub fn make_bet_user(
    player: &Player,
    req_bet: u32,
    event_pump: &mut EventPump,
    canvas: &mut Canvas<Window>,
    ttf_context: &sdl2::ttf::Sdl2TtfContext,
    game: &Game,
) -> Result<Option<u32>, String> {
    if player.chips == 0 {
        return Ok(Some(0));
    }
    let req_bet = if player.chips <= req_bet {
        player.chips
    } else {
        req_bet
    };
    let slider_pos = ControlPosition::init_control_positon(canvas).slider;
    let mut slider = Slider::new(slider_pos.x, slider_pos.y, 300, 40, req_bet as i32, player.chips as i32);
    // mogoče treba req_bet in player.chips mal bol obravnavat, da nau problemov k ma player edino možnost it all in
    let check_button = Button::init_check_button(canvas);
    let allin_button = Button::init_allin_button(canvas);
    let mut call_button = Button::init_call_button(canvas);
    let mut raise_button = Button::init_raise_button(canvas);
    let mut fold_button = Button::init_fold_button(canvas);

    let _: Vec<_> = event_pump.poll_iter().collect();
    loop {
        for event in event_pump.poll_iter() {
            // se sprehodi cez use evente
            Button::handle_button_events(&event, &mut fold_button);
            Button::handle_button_events(&event, &mut call_button);
            Button::handle_button_events(&event, &mut raise_button);
            slider.handle_event(&event);
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    SHOULD_QUIT.store(true, Ordering::Relaxed);
                    return Ok(None); // Signal za izhod
                }
                _ => {}
            }
        }
        let raise_value = slider.get_value() as u32;
        if fold_button.is_clicked {
            write_info(canvas, &format!("{:?} folded", player.id), ttf_context, 250)?;
            canvas.present();
            ::std::thread::sleep(Duration::from_millis(800));
            return Ok(None);
        } else if call_button.is_clicked {
            if req_bet <= player.chips {
                write_info(canvas, &format!("{:?} called", player.id), &ttf_context, WRITE_INFO_SIZE)?;
                canvas.present();
                ::std::thread::sleep(Duration::from_millis(800));
                return Ok(Some(req_bet));
            } else {
                write_info(
                    canvas,
                    &format!(
                        "{:?} you dont have enough chips to call full bet, you went all in",
                        player.id
                    ),
                    &ttf_context,
                    WRITE_INFO_SIZE,
                )?;
                canvas.present();
                ::std::thread::sleep(Duration::from_millis(800));
                return Ok(Some(player.chips));
            }
        } else if raise_button.is_clicked {
            if player.chips >= raise_value {
                write_info(canvas, &format!("{:?} raised", player.id), &ttf_context, WRITE_INFO_SIZE)?;
                canvas.present();
                ::std::thread::sleep(Duration::from_millis(800));
                return Ok(Some(raise_value));
            } else {
                write_info(
                    canvas,
                    &format!(
                        "{:?} you dont have enough chips, if u want to all in call",
                        player.id
                    ),
                    &ttf_context,
                    WRITE_INFO_SIZE,
                )?;
                canvas.present();
                ::std::thread::sleep(Duration::from_millis(800));
                continue;
            }
        }
        let (r, g, b) = (173, 216, 230); // Light blue color
        render_screen(canvas, Color::RGB(r, g, b), game, &ttf_context)?;
        Button::draw_button(&fold_button, canvas, &ttf_context, BUTTON_TEXT_SIZE)?;
        if req_bet > 0 {
            Button::draw_button(&call_button, canvas, &ttf_context, BUTTON_TEXT_SIZE)?;
        } else {
            Button::draw_button(&check_button, canvas, &ttf_context, BUTTON_TEXT_SIZE)?;
        }
        if player.chips >= req_bet {
            Button::draw_button(&raise_button, canvas, &ttf_context, BUTTON_TEXT_SIZE)?;
            slider.draw(canvas, ttf_context)?;
        } else {
            Button::draw_button(&allin_button, canvas, &ttf_context, BUTTON_TEXT_SIZE)?;
        }
        // render_turn_indicator(player, canvas)?;
        canvas.present();
        if fold_button.is_clicked || call_button.is_clicked || raise_button.is_clicked {
            ::std::thread::sleep(Duration::from_millis(200));
        }
        ::std::thread::sleep(Duration::from_millis(33));
    }
}

pub fn make_bet_bot(
    player: &Player,
    req_bet: u32,
    event_pump: &mut EventPump,
    canvas: &mut Canvas<Window>,
    ttf_context: &sdl2::ttf::Sdl2TtfContext,
    game: &Game,
) -> Result<Option<u32>, String> {
    let _: Vec<_> = event_pump.poll_iter().collect();
    let decision = make_decision(
        &player.hand_cards,
        &game.table_cards,
        req_bet,
        player.current_bet,
        player.chips,
    );
    let (r, g, b) = (173, 216, 230);
    if let Some(bet) = decision { // to bi se tut dal lepš!!
        render_screen(canvas, Color::RGB(r, g, b), game, &ttf_context)?;
        let string = if bet == req_bet {
            // println!("pišem write_info v send_bet ko bot dela odloćiitve");
            format!("{:?} called", player.id)
        } else {
            // println!("pišem write_info v send_bet ko bot dela odloćiitve");
            format!("{:?} raised", player.id)
        };
        let start_time = std::time::Instant::now();
        while start_time.elapsed() < Duration::from_millis(800) {
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => {
                        SHOULD_QUIT.store(true, Ordering::Relaxed);
                        return Ok(None); // Signal za izhod
                    }
                    _ => {}
                }
            }
            render_screen(canvas, Color::RGB(r, g, b), game, &ttf_context)?;
            write_info(canvas, &string, ttf_context, WRITE_INFO_SIZE)?;
            canvas.present();
            ::std::thread::sleep(Duration::from_millis(30));
        }
        return Ok(Some(bet));
    } else {
        let start_time = std::time::Instant::now();
        while start_time.elapsed() < Duration::from_millis(800) {
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => {
                        SHOULD_QUIT.store(true, Ordering::Relaxed);
                        return Ok(None); // Signal za izhod
                    }
                    _ => {}
                }
            }
            render_screen(canvas, Color::RGB(r, g, b), game, &ttf_context)?;
            write_info(canvas, &format!("{:?} folded", player.id), ttf_context, WRITE_INFO_SIZE)?;
            canvas.present();
            ::std::thread::sleep(Duration::from_millis(30));
        }
        return Ok(None);
    }
}
