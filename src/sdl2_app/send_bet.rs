use sdl2::keyboard::Keycode;
use sdl2::render::Canvas;
use sdl2::ttf::Font;
use sdl2::video::Window;
use sdl2::EventPump;
use sdl2::{event::Event, pixels::Color};
use std::sync::atomic::Ordering;
use std::time::Duration;

use crate::logic::constants::{BIG_BLIND, SHOULD_QUIT};
use crate::logic::game::Game;
use crate::logic::player::Player;
use crate::sdl2_app::render_button::Button;
use crate::sdl2_app::render_text::write_info;

use super::render_screen::render_screen;
use super::slider::Slider;
use super::tactic1::make_decision;

pub fn make_bet_player1(
    player: &Player,
    req_bet: u32,
    event_pump: &mut EventPump,
    canvas: &mut Canvas<Window>,
    font: &Font,
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
    
    let mut slider = Slider::init_raise_slider(&canvas, (req_bet + BIG_BLIND) as i32, player.chips as i32);
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
            write_info(canvas, &format!("{:?} folded", player.id), font, 250)?;
            canvas.present();
            ::std::thread::sleep(Duration::from_millis(800));
            return Ok(None);
        } else if call_button.is_clicked {
            if req_bet <= player.chips {
                write_info(canvas, &format!("{:?} called", player.id), font, 250)?;
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
                    font,
                    800,
                )?;
                canvas.present();
                ::std::thread::sleep(Duration::from_millis(800));
                return Ok(Some(player.chips));
            }
        } else if raise_button.is_clicked {
            if player.chips >= raise_value {
                write_info(canvas, &format!("{:?} raised", player.id), font, 250)?;
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
                    font,
                    400,
                )?;
                canvas.present();
                ::std::thread::sleep(Duration::from_millis(800));
                continue;
            }
        }
        let (r, g, b) = (173, 216, 230); // Light blue color
        render_screen(canvas, Color::RGB(r, g, b), game, font)?;
        Button::draw_button(&fold_button, canvas, &font)?;
        if req_bet > 0 {
            Button::draw_button(&call_button, canvas, &font)?;
        } else {
            Button::draw_button(&check_button, canvas, &font)?;
        }
        if player.chips >= req_bet {
            Button::draw_button(&raise_button, canvas, &font)?;
            slider.draw(canvas, font)?;
        } else {
            Button::draw_button(&allin_button, canvas, &font)?;
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
    font: &Font,
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
        render_screen(canvas, Color::RGB(r, g, b), game, font)?;
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
            render_screen(canvas, Color::RGB(r, g, b), game, font)?;
            write_info(canvas, &string, font, 250)?;
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
            render_screen(canvas, Color::RGB(r, g, b), game, font)?;
            write_info(canvas, &format!("{:?} folded", player.id), font, 250)?;
            canvas.present();
            ::std::thread::sleep(Duration::from_millis(30));
        }
        return Ok(None);
    }
}
