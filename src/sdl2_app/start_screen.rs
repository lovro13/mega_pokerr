use std::{sync::atomic::Ordering, time::Duration};

use sdl2::{
    event::Event,
    keyboard::Keycode,
    rect::{Point, Rect},
    render::Canvas,
    video::Window,
    EventPump,
};

use crate::logic::constants::{DATABASE_PATH, DEFAULT_PLAYER_COUNT, SHOULD_QUIT};
use crate::logic::game::{self, Game};
use crate::logic::player;
use crate::logic::save_game::{list_saved_games, load_game, save_game};
use rusqlite::Connection;

use super::{
    button::Button, constants::*, render_screen::get_screen_center, render_text::draw_text,
};

pub enum StartScreenAction {
    StartNewGame,
    LoadGame,
    Exit,
}

fn load_game_screen(
    canvas: &mut Canvas<Window>,
    event_pump: &mut EventPump,
    ttf_context: &sdl2::ttf::Sdl2TtfContext,
) -> Result<Option<Game>, String> {
    let mut conn = Connection::open(DATABASE_PATH).map_err(|e| e.to_string())?;
    let saves = list_saved_games(&conn).map_err(|e| e.to_string())?;
    let screen_center = get_screen_center(canvas);
    let mut buttons = Vec::new();
    for (i, (game_id, created_at)) in saves.iter().enumerate() {
        let y_offset = (i as i32 - saves.len() as i32 / 2)
            * (LOAD_GAME_SCREEN_BUTTON_HEIGHT as i32 + LOAD_GAME_SCREEN_BUTTON_SPACING);
        let btn = Button::new(
            screen_center + Point::new(0, y_offset),
            LOAD_GAME_SCREEN_BUTTON_HEIGHT,
            LOAD_GAME_SCREEN_BUTTON_WIDTH,
            format!("Game {} ({})", game_id, created_at),
        );
        buttons.push((btn, *game_id));
    }
    loop {
        for event in event_pump.poll_iter() {
            for (btn, _) in buttons.iter_mut() {
                Button::handle_button_events(&event, btn);
            }
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
        canvas.set_draw_color(BACKGROUND_COLOR);
        canvas.clear();
        draw_text(
            canvas,
            "Select a saved game to load:",
            Rect::from_center(
                screen_center + Point::new(0, LOAD_GAME_SCREEN_TITLE_Y),
                1,
                1,
            ),
            ttf_context,
            TITLE_SIZE / 2,
            BLACK,
            None,
            false,
        )?;
        for (btn, _) in buttons.iter_mut() {
            btn.draw_button(canvas, ttf_context, BUTTON_TEXT_SIZE)?;
        }
        canvas.present();
        for (btn, game_id) in buttons.iter() {
            if btn.is_clicked {
                let tx = conn.transaction().map_err(|e| e.to_string())?;
                if let Some(game) = load_game(*game_id, &tx).map_err(|e| e.to_string())? {
                    return Ok(Some(game));
                }
            }
        }
        ::std::thread::sleep(Duration::from_millis(30));
    }
}

pub fn start_screen_state(
    canvas: &mut Canvas<Window>,
    event_pump: &mut EventPump,
    ttf_context: &sdl2::ttf::Sdl2TtfContext,
) -> Result<StartScreenAction, String> {
    let screen_center = get_screen_center(canvas);
    let button_spacing = START_SCREEN_BUTTON_SPACING;
    let mut start_button = Button::new(
        screen_center + Point::new(0, -button_spacing),
        START_BUTTON_HEIGHT,
        START_BUTTON_WIDTH,
        String::from("NEW GAME"),
    );
    let mut load_game_button = Button::new(
        screen_center,
        START_BUTTON_HEIGHT,
        START_BUTTON_WIDTH,
        String::from("LOAD GAME"),
    );
    let mut exit_button = Button::new(
        screen_center + Point::new(0, button_spacing),
        START_BUTTON_HEIGHT,
        START_BUTTON_WIDTH,
        String::from(EXIT_TEXT),
    );
    loop {
        for event in event_pump.poll_iter() {
            Button::handle_button_events(&event, &mut start_button);
            Button::handle_button_events(&event, &mut load_game_button);
            Button::handle_button_events(&event, &mut exit_button);
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    SHOULD_QUIT.store(true, Ordering::Relaxed);
                    return Ok(StartScreenAction::Exit);
                }
                _ => {}
            }
        }
        if exit_button.is_clicked {
            SHOULD_QUIT.store(true, Ordering::Relaxed);
            log::info!("Exit button clicked");
            return Ok(StartScreenAction::Exit);
        } else if start_button.is_clicked {
            // Create and save a new game
            let player_list = player::Player::init_players_with_count(DEFAULT_PLAYER_COUNT);
            let game = game::init_game(player_list);
            let mut conn = Connection::open(DATABASE_PATH).map_err(|e| e.to_string())?;
            save_game(&game.borrow(), &mut conn).map_err(|e| e.to_string())?;
            log::info!("New game created and saved");
            // You may want to return the game object or start the game loop here
            return Ok(StartScreenAction::StartNewGame);
        } else if load_game_button.is_clicked {
            if let Some(_game) = load_game_screen(canvas, event_pump, ttf_context)? {
                // You may want to return the loaded game object or start the game loop here
                return Ok(StartScreenAction::LoadGame);
            }
        }
        canvas.set_draw_color(BACKGROUND_COLOR);
        canvas.clear();
        // Draw title
        draw_text(
            canvas,
            GAME_TITLE,
            Rect::from_center(screen_center + Point::from(TITLE_POS), 1, 1),
            ttf_context,
            TITLE_SIZE,
            BLACK,
            None,
            false,
        )?;
        // Draw buttons
        start_button
            .draw_button(canvas, &ttf_context, START_SCREEN_TEXT_SIZE)
            .unwrap();
        load_game_button
            .draw_button(canvas, &ttf_context, START_SCREEN_TEXT_SIZE)
            .unwrap();
        exit_button
            .draw_button(canvas, &ttf_context, START_SCREEN_TEXT_SIZE)
            .unwrap();
        // Draw cards for visual flair
        CARD.draw_card(
            canvas,
            screen_center + Point::from(RIGHT_CARD),
            true,
            ANGLE,
            CARD_SIZE,
        )
        .unwrap();
        CARD.draw_card(
            canvas,
            screen_center + Point::from(LEFT_CARD),
            true,
            ANGLE,
            CARD_SIZE,
        )
        .unwrap();
        canvas.present();
        ::std::thread::sleep(Duration::from_millis(30));
    }
}
