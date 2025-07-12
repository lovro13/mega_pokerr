use std::{sync::atomic::Ordering, time::Duration};

use sdl2::{
    event::Event,
    keyboard::Keycode,
    rect::{Point, Rect},
    render::Canvas,
    video::Window,
    EventPump,
};

use crate::logic::constants::{DEFAULT_PLAYER_COUNT, MAX_PLAYERS, MIN_PLAYERS, SHOULD_QUIT};

use super::{
    constants::*, render_button::Button, render_screen::get_screen_center, render_text::draw_text
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

pub fn settings_screen_state(
    canvas: &mut Canvas<Window>,
    event_pump: &mut EventPump,
    ttf_context: &sdl2::ttf::Sdl2TtfContext,
    settings: &mut GameSettings,
) -> Result<bool, String> {
    let screen_center = get_screen_center(canvas);
    
    let mut back_button = Button::new(
        screen_center + Point::from((0, 200)),
        SETTINGS_BUTTON_HEIGHT,
        SETTINGS_BUTTON_WIDTH,
        String::from(BACK_TEXT),
    );
    
    let mut apply_button = Button::new(
        screen_center + Point::from((0, 100)),
        SETTINGS_BUTTON_HEIGHT,
        SETTINGS_BUTTON_WIDTH,
        String::from(APPLY_TEXT),
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
            SETTINGS_TITLE_SIZE,
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
            SETTINGS_FONT_SIZE,
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
            SETTINGS_FONT_SIZE,
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
            .draw_button(canvas, &ttf_context, SETTINGS_FONT_SIZE)
            .unwrap();
        apply_button
            .draw_button(canvas, &ttf_context, SETTINGS_FONT_SIZE)
            .unwrap();

        canvas.present();
        ::std::thread::sleep(Duration::from_millis(30));
    }
} 