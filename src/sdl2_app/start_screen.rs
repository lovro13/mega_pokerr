use std::{sync::atomic::Ordering, time::Duration};

use sdl2::{
    event::Event,
    keyboard::Keycode,
    rect::{Point, Rect},
    render::Canvas,
    video::Window,
    EventPump,
};

use crate::logic::constants::SHOULD_QUIT;

use super::{
    constants::{BACKGROUND_COLOR, BLACK}, positions::{ANGLE, CARD, CARD_SIZE, EXIT_BUTTON, LEFT_CARD, RIGHT_CARD, START_BUTTON, START_BUTTON_HEIGHT, START_BUTTON_WIDTH, START_SCREEN_TEXT_SIZE, TITLE_POS, TITLE_SIZE}, render_button::Button, render_screen::get_screen_center, render_text::draw_text
};


pub fn start_screen_state(
    canvas: &mut Canvas<Window>,
    event_pump: &mut EventPump,
    ttf_context: &sdl2::ttf::Sdl2TtfContext,
) -> Result<bool, String> {
    let screen_center = get_screen_center(canvas);
    let mut start_button = Button::new(
        screen_center + Point::from(START_BUTTON),
        START_BUTTON_HEIGHT,
        START_BUTTON_WIDTH,
        String::from("START GAME"),
    );
    let mut exit_button = Button::new(
        screen_center + Point::from(EXIT_BUTTON),
        START_BUTTON_HEIGHT,
        START_BUTTON_WIDTH,
        String::from("EXIT"),
    );

    let screen_center = get_screen_center(canvas);
    loop {
        for event in event_pump.poll_iter() {
            Button::handle_button_events(&event, &mut start_button);
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
        } else if start_button.is_clicked {
            return Ok(true);
        }
        canvas.set_draw_color(BACKGROUND_COLOR);
        canvas.clear();
        start_button
            .draw_button(canvas, &ttf_context, START_SCREEN_TEXT_SIZE)
            .unwrap();
        exit_button
            .draw_button(canvas, &ttf_context, START_SCREEN_TEXT_SIZE)
            .unwrap();

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
        draw_text(
            canvas,
            "MEGA POKER",
            Rect::from_center(screen_center + Point::from(TITLE_POS), 1, 1),
            ttf_context,
            TITLE_SIZE,
            BLACK,
            None,
            false,
        )?;
        canvas.present();
        ::std::thread::sleep(Duration::from_millis(30));
    }
}
