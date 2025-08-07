use std::{sync::atomic::Ordering, time::Duration};

use rusqlite::Connection;
use sdl2::{
    event::Event,
    keyboard::Keycode,
    rect::{Point, Rect},
    render::{Canvas, WindowCanvas},
    video::Window,
    EventPump,
};

use crate::logic::{
    constants::{DATABASE_PATH, DEFAULT_PLAYER_COUNT, MAX_PLAYERS, MIN_PLAYERS, SHOULD_QUIT},
    game::Game,
};

use super::{
    button::Button, constants::*, render_screen::get_screen_center, render_text::draw_text,
    save_game,
};

#[derive(Debug, Clone)]
pub struct GameSettings {
    pub player_count: usize,
}

impl Default for GameSettings {
    fn default() -> Self {
        Self {
            player_count: DEFAULT_PLAYER_COUNT,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MenuAction {
    None,
    Resume,
    Save,
    Exit,
    ExitToMainMenu,
}

pub fn new_game_start_screen_state(
    canvas: &mut Canvas<Window>,
    event_pump: &mut EventPump,
    ttf_context: &sdl2::ttf::Sdl2TtfContext,
    settings: &mut GameSettings,
) -> Result<bool, String> {
    let screen_center = get_screen_center(canvas);

    let mut back_button = Button::new(
        screen_center + Point::from((0, 200)),
        SETTINGS_START_BUTTON_HEIGHT,
        SETTINGS_START_BUTTON_WIDTH,
        String::from(BACK_TEXT),
    );

    let mut apply_button = Button::new(
        screen_center + Point::from(APPLY_BUTTON_POS),
        SETTINGS_START_BUTTON_HEIGHT,
        SETTINGS_START_BUTTON_WIDTH,
        String::from(APPLY_AND_START_NEW_GAME_TEXT),
    );

    let mut player_count = settings.player_count;

    loop {
        for event in event_pump.poll_iter() {
            Button::handle_button_events(&event, &mut back_button);
            Button::handle_button_events(&event, &mut apply_button);

            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    SHOULD_QUIT.store(true, Ordering::Relaxed);
                    return Ok(false);
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    ..
                } => {
                    if player_count < MAX_PLAYERS {
                        player_count += 1;
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => {
                    if player_count > MIN_PLAYERS {
                        player_count -= 1;
                    }
                }
                _ => {}
            }
        }

        if back_button.is_clicked {
            return Ok(false);
        } else if apply_button.is_clicked {
            settings.player_count = player_count;
            log::info!("Settings applied: player_count = {}", player_count);
            return Ok(true);
        }

        canvas.set_draw_color(BACKGROUND_COLOR);
        canvas.clear();

        // Nariši naslov
        draw_text(
            canvas,
            "SETTINGS",
            Rect::from_center(screen_center + Point::from((0, -200)), 1, 1),
            ttf_context,
            SETTINGS_START_TITLE_SIZE,
            BLACK,
            None,
            false,
        )?;

        // Nariši label za število igralcev
        draw_text(
            canvas,
            PLAYER_COUNT_LABEL,
            Rect::from_center(screen_center + Point::from((-200, -50)), 1, 1),
            ttf_context,
            SETTINGS_START_FONT_SIZE,
            BLACK,
            None,
            false,
        )?;

        // Nariši trenutno število igralcev
        draw_text(
            canvas,
            &player_count.to_string(),
            Rect::from_center(screen_center + Point::from((200, -50)), 1, 1),
            ttf_context,
            SETTINGS_START_FONT_SIZE,
            BLACK,
            None,
            false,
        )?;

        // Nariši navodila
        // draw_text(
        //     canvas,
        //     "Use UP/DOWN arrows to change player count",
        //     Rect::from_center(screen_center + Point::from((0, 0)), 1, 1),
        //     ttf_context,
        //     30,
        //     BLACK,
        //     None,
        //     false,
        // )?;

        back_button
            .draw_button(canvas, &ttf_context, SETTINGS_START_FONT_SIZE)
            .unwrap();
        apply_button
            .draw_button(canvas, &ttf_context, SETTINGS_START_FONT_SIZE)
            .unwrap();

        canvas.present();
        ::std::thread::sleep(Duration::from_millis(30));
    }
}

pub fn menu_screen_render(
    canvas: &mut WindowCanvas,
    resume_button: &mut Button,
    save_button: &mut Button,
    exit_to_start_screen_button: &mut Button,
    return_to_start_button: &mut Button,
    ttf_context: &sdl2::ttf::Sdl2TtfContext,
) -> Result<(), String> {
    let screen_center = get_screen_center(canvas);

    pub const MENU_POS: (i32, i32) = (0, (SETTINGS_BUTTON_HEIGTH / 2) as i32 + 10);
    let background_rect = Rect::from_center(
        screen_center + Point::from(MENU_POS),
        SETTINGS_WINDOW_WIDTH,
        SETTINGS_WINDOW_HEIGHT,
    );
    canvas.set_draw_color(DARK_BLUE);
    canvas.fill_rect(background_rect)?;
    resume_button.draw_button(canvas, ttf_context, SETTINGS_FONT_SIZE)?;
    save_button.draw_button(canvas, ttf_context, SETTINGS_FONT_SIZE)?;
    exit_to_start_screen_button.draw_button(canvas, ttf_context, SETTINGS_FONT_SIZE)?;
    return_to_start_button.draw_button(canvas, ttf_context, SETTINGS_FONT_SIZE)?;
    Ok(())
}

pub fn menu_screen_handle_events(
    event: &Event,
    resume_button: &mut Button,
    save_button: &mut Button,
    exit_button: &mut Button,
    return_to_start_button: &mut Button,
    game: &Game,
    settings_window: &mut bool,
) -> Result<MenuAction, String> {
    Button::handle_button_events(event, resume_button);
    Button::handle_button_events(event, save_button);
    Button::handle_button_events(event, exit_button);
    Button::handle_button_events(event, return_to_start_button);
    if resume_button.is_clicked {
        *settings_window = false;
        return Ok(MenuAction::Resume);
    }
    if save_button.is_clicked {
        let mut connection = Connection::open(DATABASE_PATH).unwrap();
        let _ = save_game::save_game(game, &mut connection).unwrap();
        return Ok(MenuAction::Save);
    }
    if exit_button.is_clicked {
        *settings_window = false;
        crate::logic::constants::SHOULD_QUIT.store(true, Ordering::Relaxed);
        return Ok(MenuAction::Exit);
    }
    if return_to_start_button.is_clicked {
        crate::logic::constants::SHOULD_RETURN_TO_START
            .store(true, std::sync::atomic::Ordering::Relaxed);
        *settings_window = false;
        return Ok(MenuAction::ExitToMainMenu);
    }
    Ok(MenuAction::None)
}
