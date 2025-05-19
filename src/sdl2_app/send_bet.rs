use sdl2::keyboard::Keycode;
use sdl2::render::Canvas;
use sdl2::ttf::Font;
use sdl2::video::Window;
use sdl2::EventPump;
use sdl2::{event::Event, pixels::Color};
use std::sync::atomic::Ordering;
use std::time::Duration;

use crate::logic::card::Card;
use crate::logic::constants::BIG_BLIND;
use crate::logic::constants::SHOULD_QUIT;
use crate::logic::game::Game;
use crate::logic::player::Player;
use crate::sdl2_app::render_button::Button;
use crate::sdl2_app::render_text::write_info;

use super::render_screen::render_screen;
use super::slider::Slider;
use super::tactic1::rank_cards_preflop;

pub fn make_bet_player1(
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
    let mut slider = Slider::new(1150, 840, 600, 20, req_bet, player.chips);
    loop {
        for event in event_pump.poll_iter() {
            // se sprehodi cez use evente
            Button::handle_button_events(&event, fold_button);
            Button::handle_button_events(&event, call_button);
            Button::handle_button_events(&event, raise_button);
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
        let raise_value = slider.get_value();
        if fold_button.is_clicked {
            write_info(canvas, &format!("{:?} folded", player.name), font, 250)?;
            canvas.present();
            ::std::thread::sleep(Duration::from_millis(800));
            return Ok(None);
        } else if call_button.is_clicked {
            if req_bet <= player.chips {
                write_info(canvas, &format!("{:?} called", player.name), font, 250)?;
                canvas.present();
                ::std::thread::sleep(Duration::from_millis(800));
                return Ok(Some(req_bet));
            } else {
                write_info(
                    canvas,
                    &format!(
                        "{:?} you dont have enough chips to call full bet, you went all in",
                        player.name
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
                write_info(canvas, &format!("{:?} raised", player.name), font, 250)?;
                canvas.present();
                ::std::thread::sleep(Duration::from_millis(800));
                return Ok(Some(raise_value));
            } else {
                write_info(
                    canvas,
                    &format!(
                        "{:?} you dont have enough chips, if u want to all in call",
                        player.name
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
        slider.draw(canvas, font)?;
        Button::draw_button(&fold_button, canvas, &font)?;
        Button::draw_button(&call_button, canvas, &font)?;
        Button::draw_button(&raise_button, canvas, &font)?;
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
        req_bet,
        player.current_bet,
        player.chips,
    );
    let (r, g, b) = (173, 216, 230);
    if let Some(bet) = decision {
        render_screen(canvas, Color::RGB(r, g, b), game, font)?;
        let string = 
        if bet == req_bet {
            // println!("pišem write_info v send_bet ko bot dela odloćiitve");
            format!("{:?} called", player.name)
        } else {
            // println!("pišem write_info v send_bet ko bot dela odloćiitve");
            format!("{:?} raised", player.name)
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
            write_info(canvas, &format!("{:?} folded", player.name), font, 250)?;
            canvas.present();
            ::std::thread::sleep(Duration::from_millis(30));
        }
        return Ok(None);
    }
}

pub fn make_decision(
    player_cards: &(Card, Card),
    req_bet: u32,
    curr_bet: u32,
    player_chips: u32,
) -> Option<u32> {
    let hand_cards_vec: Vec<_> = vec![player_cards.0.clone(), player_cards.1.clone()];
    let rank_points = rank_cards_preflop(hand_cards_vec);
    // println!("hand ranking of cards {} {} is {}", player_cards.0, player_cards.1, rank_points);
    if (rank_points < 10) || (rank_points < 25 && curr_bet == 0) {
        if req_bet > player_chips {
            return Some(player_chips);
        }
        if curr_bet <= 5 * BIG_BLIND {
            Some(5 * BIG_BLIND - curr_bet)
        } else {
            Some(0)
        }
    } else if rank_points < 35 && player_chips <= req_bet {
        Some(req_bet)
    } else if req_bet == curr_bet {
        return Some(0);
    } else {
        None
    }
}
