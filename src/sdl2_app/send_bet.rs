use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::{Canvas, WindowCanvas};
use sdl2::video::Window;
use sdl2::EventPump;
use std::sync::atomic::Ordering;
use std::time::Duration;

use crate::logic::constants::{BIG_BLIND, SHOULD_QUIT, SHOULD_RETURN_TO_START};
use crate::logic::game::Game;
use crate::logic::player::Player;
use crate::sdl2_app::button::Button;
use crate::sdl2_app::constants::*;
use crate::sdl2_app::menu::{menu_screen_handle_events, menu_screen_render};
use crate::sdl2_app::render_text::write_info;

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
        req_bet: u32
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
    player_count: usize,
) -> Result<Option<u32>, String> {
    // Check if player is a bot (Player1 is the main player, others are bots)
    let is_bot = player.id != MAIN_PLAYER;
    log::debug!(
        "Player {:?} making bet (is_bot: {}, req_bet: {}, chips: {})",
        player.id,
        is_bot,
        req_bet,
        player.chips
    );

    if player.chips == 0 {
        log::debug!("Player {:?} has no chips, returning 0", player.id);
        return Ok(Some(0));
    }

    // Initialize state based on player type
    let mut state = if is_bot {
        log::debug!("Bot {:?} making decision", player.id);
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
            } else if bet == 0 {
                format!("{:?} checked", player.id)
            } else if bet == player.chips {
                format!("{:?} went all in", player.id)
            } else {
                format!("{:?} raised", player.id)
            }
        } else {
            format!("{:?} folded", player.id)
        };

        log::info!("Bot {:?} decision: {}", player.id, message);

        BetState::BotBet {
            decision,
            start_time: std::time::Instant::now(),
            message,
        }
    } else {
        log::debug!("Human player {:?} making bet", player.id);
        let req_bet = if player.chips <= req_bet {
            player.chips
        } else {
            req_bet
        };

        BetState::UserBet {
            slider: Slider::init_raise_slider(
                &canvas,
                (req_bet + BIG_BLIND) as i32,
                player.chips as i32,
            ),
            check_button: Button::init_check_button(canvas),
            allin_button: Button::init_allin_button(canvas),
            call_button: Button::init_call_button(canvas),
            raise_button: Button::init_raise_button(canvas),
            fold_button: Button::init_fold_button(canvas),
            req_bet
        }
    };

    let _: Vec<_> = event_pump.poll_iter().collect();
    let mut settings_button = Button::init_settings_button();
    let mut settings_window = false;
    let mut resume_button = Button::init_resume_button(canvas);
    let mut save_button = Button::init_save_button(canvas);
    let mut exit_button = Button::init_exit_button(canvas);
    let mut return_to_start_button = Button::init_return_to_main_menu_button(canvas);

    loop {
        // Handle events
        for event in event_pump.poll_iter() {
            Button::handle_button_events(&event, &mut settings_button);
            if settings_window {
                let act = menu_screen_handle_events(
                    &event,
                    &mut resume_button,
                    &mut save_button,
                    &mut exit_button,
                    &mut return_to_start_button,
                    game,
                    &mut settings_window,
                )?;
                match act {
                    crate::sdl2_app::menu::MenuAction::ExitToMainMenu => {
                        SHOULD_RETURN_TO_START.store(true, Ordering::Relaxed);
                        return Ok(Some(0));
                    }
                    _ => {}
                }
            }
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
                BetState::UserBet {
                    slider,
                    check_button,
                    allin_button,
                    call_button,
                    raise_button,
                    fold_button,
                    ..
                } => {
                    Button::handle_button_events(&event, fold_button);
                    Button::handle_button_events(&event, call_button);
                    Button::handle_button_events(&event, raise_button);
                    Button::handle_button_events(&event, check_button);
                    Button::handle_button_events(&event, allin_button);
                    slider.handle_event(&event);
                }
                BetState::BotBet { .. } => {
                    // Bot doesn't handle user input events
                }
            }
        }
        if settings_window {
            render_screen(canvas, BACKGROUND_COLOR, game, ttf_context, player_count)?;
            menu_screen_render(
                canvas,
                &mut resume_button,
                &mut save_button,
                &mut exit_button,
                &mut return_to_start_button,
                ttf_context,
            )?;
            canvas.present();
            continue;
        }

        // Process state and check for completion
        match &mut state {
            BetState::UserBet {
                slider,
                check_button,
                allin_button,
                call_button,
                raise_button,
                fold_button,
                req_bet
            } => {
                let bet = user_bet(
                    slider,
                    fold_button,
                    canvas,
                    player,
                    ttf_context,
                    call_button,
                    *req_bet,
                    raise_button,
                    game,
                    player_count,
                    check_button,
                    allin_button,
                );
                settings_button.draw_button(canvas, ttf_context, BUTTON_FONT_SIZE)?;
                if settings_button.is_clicked {
                    settings_window = true;
                }
                canvas.present();
                    match bet {
                        Ok(a) => {
                            return Ok(a);
                        }
                        Err(e) => {
                            if e == String::from("CONTINUE") {
                                continue;
                            } else {
                                return Err(e);
                            }
                        }
                    }
            }

            BetState::BotBet {
                decision,
                start_time,
                message,
            } => {
                if start_time.elapsed() >= Duration::from_millis(ANIMATION_DURATION_MS) {
                    return Ok(*decision);
                }

                // Render bot decision animation
                render_screen(canvas, LIGHT_BLUE, game, &ttf_context, player_count)?;
                write_info(canvas, message, ttf_context, WRITE_INFO_SIZE)?;
                ::std::thread::sleep(Duration::from_millis(BOT_DECISION_DELAY_MS));
            }
        }
        settings_button.draw_button(canvas, ttf_context, BUTTON_FONT_SIZE)?;
        if settings_button.is_clicked {
            settings_window = true;
        }
        canvas.present();
        ::std::thread::sleep(Duration::from_millis(FRAME_DURATION_MS));
    }
}

pub fn user_bet(
    slider: &Slider, // dont need mut button or slider anywhere i just need to read them but they are defined mutable
    fold_button: &Button,
    canvas: &mut WindowCanvas,
    player: &Player,
    ttf_context: &sdl2::ttf::Sdl2TtfContext,
    call_button: &Button,
    req_bet: u32,
    raise_button: &Button,
    game: &Game,
    player_count: usize,
    check_button: &Button,
    allin_button: &Button,
) -> Result<Option<u32>, String> {
    let raise_value = slider.get_value() as u32;

    if fold_button.is_clicked {
        write_info(
            canvas,
            &format!("{:?} folded", player.id),
            ttf_context,
            WRITE_INFO_SIZE,
        )?;
       return Ok(None);
    } else if call_button.is_clicked {
        if req_bet <= player.chips {
            write_info(
                canvas,
                &format!("{:?} called", player.id),
                &ttf_context,
                WRITE_INFO_SIZE,
            )?;
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
            return Ok(Some(player.chips));
        }
    } else if raise_button.is_clicked {
        if player.chips >= raise_value {
            write_info(
                canvas,
                &format!("{:?} raised", player.id),
                &ttf_context,
                WRITE_INFO_SIZE,
            )?;
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
            return Err(String::from("CONTINUE"));
        }
    }

    // Render user interface
    render_screen(canvas, LIGHT_BLUE, game, &ttf_context, player_count)?;
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

    if fold_button.is_clicked || call_button.is_clicked || raise_button.is_clicked {
        ::std::thread::sleep(Duration::from_millis(SHORT_ANIMATION_DURATION_MS));
    }
    return Err(String::from("CONTINUE"));
}
