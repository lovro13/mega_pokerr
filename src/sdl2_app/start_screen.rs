use std::{sync::atomic::Ordering, time::Duration};

use sdl2::{event::Event, keyboard::Keycode, pixels::Color, rect::Point, render::Canvas, ttf::Font, video::Window, EventPump};

use crate::logic::constants::SHOULD_QUIT;

use super::{constants::BACKGROUND_COLOR, render_button::Button, render_screen::get_screen_center};



pub fn start_screen_state(canvas: &mut Canvas<Window>, event_pump: &mut EventPump, font: &Font) -> bool {
    let screen_center = get_screen_center(canvas);
    let mut start_button = Button::new(&screen_center, 100, 600, String::from("START GAME"));
    let mut exit_button = Button::new(&(screen_center + Point::new(0, 110)), 100, 600, String::from("EXIT"));
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
                        return false; // Signal za izhod
                    }
                    _ => {}
                }
        }
        if exit_button.is_clicked {
            SHOULD_QUIT.store(true, Ordering::Relaxed);
            return false;
        } else if start_button.is_clicked {
            return true;
        }
        canvas.set_draw_color(Color::RGB(BACKGROUND_COLOR.0, BACKGROUND_COLOR.1, BACKGROUND_COLOR.2));
        canvas.clear();
        start_button.draw_button(canvas, &font).unwrap();
        exit_button.draw_button(canvas, &font).unwrap();
        canvas.present();
        ::std::thread::sleep(Duration::from_millis(30));
    }
}