use sdl2::keyboard::Keycode;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::EventPump;
use sdl2::event::Event;
use std::sync::atomic::Ordering;
use std::time::Duration;

use crate::logic::constants::{BIG_BLIND, SHOULD_QUIT};
use crate::logic::game::Game;
use crate::logic::player::Player;
use crate::sdl2_app::render_button::Button;
use crate::sdl2_app::render_text::write_info;
use crate::sdl2_app::constants::*;

use super::render_screen::render_screen;
use super::slider::Slider;
use super::tactic1::make_decision;

enum BetState {
    UserBet {
        slider: Slider,
        check_button: Button,
        allin_button: Button,
        call_button: Button,
        raise_button: Button,
        fold_button: Button,
        req_bet: u32,
    },
    BotBet {
        decision: Option<u32>,
        start_time: std::time::Instant,
        message: String,
    },
}

pub fn make_bet(
    player: &Player,
    req_bet: u32,
    event_pump: &mut EventPump,
    canvas: &mut Canvas<Window>,
    ttf_context: &sdl2::ttf::Sdl2TtfContext,
    game: &Game,
) -> Result<Option<u32>, String> {
    // Check if player is a bot (Player1 is the main player, others are bots)
    let is_bot = player.id != MAIN_PLAYER;
    
    if player.chips == 0 {
        return Ok(Some(0));
    }

    // Initialize state based on player type
    let mut state = if is_bot {
        let decision = make_decision(
            &player.hand_cards,
            &game.table_cards,
            req_bet,
            player.current_bet,
            player.chips,
        );
        
        let message = if let Some(bet) = decision {
            if bet == req_bet {
                format!("{:?} called", player.id)
            } else {
                format!("{:?} raised", player.id)
            }
        } else {
            format!("{:?} folded", player.id)
        };
        
        BetState::BotBet {
            decision,
            start_time: std::time::Instant::now(),
            message,
        }
    } else {
        let req_bet = if player.chips <= req_bet {
            player.chips
        } else {
            req_bet
        };
        
        BetState::UserBet {
            slider: Slider::init_raise_slider(&canvas, (req_bet + BIG_BLIND) as i32, player.chips as i32),
            check_button: Button::init_check_button(canvas),
            allin_button: Button::init_allin_button(canvas),
            call_button: Button::init_call_button(canvas),
            raise_button: Button::init_raise_button(canvas),
            fold_button: Button::init_fold_button(canvas),
            req_bet,
        }
    };

    let _: Vec<_> = event_pump.poll_iter().collect();
    
    loop {
        // Handle events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    SHOULD_QUIT.store(true, Ordering::Relaxed);
                    return Ok(None);
                }
                _ => {}
            }
            
            // Handle state-specific events
            match &mut state {
                BetState::UserBet { slider, call_button, raise_button, fold_button, .. } => {
                    Button::handle_button_events(&event, fold_button);
                    Button::handle_button_events(&event, call_button);
                    Button::handle_button_events(&event, raise_button);
                    slider.handle_event(&event);
                }
                BetState::BotBet { .. } => {
                    // Bot doesn't handle user input events
                }
            }
        }
        
        // Process state and check for completion
        match &mut state {
            BetState::UserBet { slider, check_button, allin_button, call_button, raise_button, fold_button, req_bet } => {
                let raise_value = slider.get_value() as u32;
                
                if fold_button.is_clicked {
                    write_info(canvas, &format!("{:?} folded", player.id), ttf_context, WRITE_INFO_SIZE)?;
                    canvas.present();
                    ::std::thread::sleep(Duration::from_millis(ANIMATION_DURATION_MS));
                    return Ok(None);
                } else if call_button.is_clicked {
                    if *req_bet <= player.chips {
                        write_info(canvas, &format!("{:?} called", player.id), &ttf_context, WRITE_INFO_SIZE)?;
                        canvas.present();
                        ::std::thread::sleep(Duration::from_millis(ANIMATION_DURATION_MS));
                        return Ok(Some(*req_bet));
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
                        ::std::thread::sleep(Duration::from_millis(ANIMATION_DURATION_MS));
                        return Ok(Some(player.chips));
                    }
                } else if raise_button.is_clicked {
                    if player.chips >= raise_value {
                        write_info(canvas, &format!("{:?} raised", player.id), &ttf_context, WRITE_INFO_SIZE)?;
                        canvas.present();
                        ::std::thread::sleep(Duration::from_millis(ANIMATION_DURATION_MS));
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
                        ::std::thread::sleep(Duration::from_millis(ANIMATION_DURATION_MS));
                        continue;
                    }
                }
                
                // Render user interface
                render_screen(canvas, game, &ttf_context)?;
                Button::draw_button(&fold_button, canvas, &ttf_context, BUTTON_TEXT_SIZE)?;
                if *req_bet > 0 {
                    Button::draw_button(&call_button, canvas, &ttf_context, BUTTON_TEXT_SIZE)?;
                } else {
                    Button::draw_button(&check_button, canvas, &ttf_context, BUTTON_TEXT_SIZE)?;
                }
                if player.chips >= *req_bet {
                    Button::draw_button(&raise_button, canvas, &ttf_context, BUTTON_TEXT_SIZE)?;
                    slider.draw(canvas, ttf_context)?;
                } else {
                    Button::draw_button(&allin_button, canvas, &ttf_context, BUTTON_TEXT_SIZE)?;
                }
                canvas.present();
                
                if fold_button.is_clicked || call_button.is_clicked || raise_button.is_clicked {
                    ::std::thread::sleep(Duration::from_millis(SHORT_ANIMATION_DURATION_MS));
                }
            }
            
            BetState::BotBet { decision, start_time, message } => {
                if start_time.elapsed() >= Duration::from_millis(ANIMATION_DURATION_MS) {
                    return Ok(*decision);
                }
                
                // Render bot decision animation
                render_screen(canvas, game, &ttf_context)?;
                write_info(canvas, message, ttf_context, WRITE_INFO_SIZE)?;
                canvas.present();
                ::std::thread::sleep(Duration::from_millis(BOT_DECISION_DELAY_MS));
            }
        }
        
        ::std::thread::sleep(Duration::from_millis(FRAME_DURATION_MS));
    }
}
