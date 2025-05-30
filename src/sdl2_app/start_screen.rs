use std::{sync::atomic::Ordering, time::Duration};

use sdl2::{
    event::Event,
    keyboard::Keycode,
    rect::{Point, Rect},
    render::Canvas,
    video::Window,
    EventPump,
};

use crate::logic::{card::Card, constants::SHOULD_QUIT};

use super::{
    constants::{BACKGROUND_COLOR, BLACK},
    render_button::Button,
    render_screen::get_screen_center,
    render_text::draw_text,
};

const START_BUTTON: (i32, i32) = (0, 0);
const EXIT_BUTTON: (i32, i32) = (0, 150);
const START_BUTTON_HEIGHT: u32 = 100;
const START_BUTTON_WIDTH: u32 = 600;
const START_SCREEN_TEXT_SIZE: u16 = 80;

const CARD_SIZE: f64 = 3.;

const RIGHT_CARD: (i32, i32) = (500, 0);
const LEFT_CARD: (i32, i32) = (-500, 0);
const CARD: Card = Card {
    number: crate::logic::card::CardNumber::NA,
    color: crate::logic::card::CardColor::Hearts,
};
const ANGLE: f64 = 20.;

const TITLE_SIZE: u16 = 120;
const TITLE_POS: (i32, i32) = (0, -200);

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
