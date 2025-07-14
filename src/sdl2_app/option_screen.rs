use std::{sync::atomic::Ordering, time::Duration};

use sdl2::{event::Event, keyboard::Keycode, rect::Point, render::WindowCanvas, EventPump};

use crate::logic::{constants::SHOULD_QUIT, game::Game};

use super::{
    constants::*,
    button::Button,
    render_screen::{get_screen_center, render_screen},
};

pub fn option_state(
    canvas: &mut WindowCanvas,
    event_pump: &mut EventPump,
    ttf_context: &sdl2::ttf::Sdl2TtfContext,
    game: &Game,
) -> Result<bool, String> {
    let screen_center = get_screen_center(canvas);
    let mut resume_button = Button::new(
        screen_center + Point::from(START_BUTTON),
        START_BUTTON_HEIGHT,
        START_BUTTON_WIDTH,
        String::from(START_GAME_TEXT),
    );
    let mut exit_button = Button::new(
        screen_center + Point::from(EXIT_BUTTON),
        START_BUTTON_HEIGHT,
        START_BUTTON_WIDTH,
        String::from(EXIT_TEXT),
    );

    let screen_center = get_screen_center(canvas);
    loop {
        for event in event_pump.poll_iter() {
            Button::handle_button_events(&event, &mut resume_button);
            Button::handle_button_events(&event, &mut exit_button);
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    SHOULD_QUIT.store(true, Ordering::Relaxed);
                    return Ok(false); // Signal za izhod
                }
                _ => {}
            }
        }
        if exit_button.is_clicked {
            SHOULD_QUIT.store(true, Ordering::Relaxed);
            return Ok(false);
        } else if resume_button.is_clicked {
            return Ok(true);
        }

        render_screen(canvas, BACKGROUND_COLOR, game, ttf_context, 6)?; // Privzeto 6 igralcev
                                                                        // Set the draw color to DARK_BLUE
        canvas.set_draw_color(DARK_BLUE);

        // Calculate the center of the screen and the rect's top-left corner

        let rect = sdl2::rect::Rect::from_center(screen_center, RECT_WIDTH, RECT_HEIGHT);
        canvas.fill_rect(rect)?;
        resume_button
            .draw_button(canvas, &ttf_context, START_SCREEN_TEXT_SIZE)
            .unwrap();
        exit_button
            .draw_button(canvas, &ttf_context, START_SCREEN_TEXT_SIZE)
            .unwrap();

        canvas.present();
        ::std::thread::sleep(Duration::from_millis(30));
    }
}
