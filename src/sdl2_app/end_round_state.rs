use std::time::Duration;

use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;
use sdl2::ttf::Font;
use sdl2::{event::Event, EventPump};

use crate::logic::{choose_winner::choose_winner, game::Game};
use crate::sdl2_app::render_text::write_info;
use crate::sdl2_app::render_button::Button;

use super::render_screen::render_screen;

pub fn end_round(
    game: &mut Game,
    event_pump: &mut EventPump,
    canvas: &mut WindowCanvas,
    font: &Font,
) -> Result<(), String> {
    println!("do tukaj pridm");
    let curr_pot = game.pot;
    let mut winners = choose_winner(game);
    let winnings = curr_pot - winners.len() as u32;
    let mut print_winners = vec![];
    for winner in winners.iter_mut() {
        winner.chips += winnings;
        print_winners.push(winner.name.clone());
    }

    let mut continue_button = Button::init_end_of_round_button(canvas);
    loop {
        for event in event_pump.poll_iter() {
            // se sprehodi cez use evente
            Button::handle_button_events(&event, &mut continue_button);
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    panic!("ustavljam igro z panic, ker drugače ne gre :) <3");
                }
                _ => {}
            }
        }
        let (r, g, b) = (173, 216, 230); // Light blue color
        println!("tukaj sem1");
        render_screen(canvas, Color::RGB(r, g, b), game, font)?;
        println!("tukaj sem2");
        write_info(
            canvas,
            format!("{:?} won the round and {} chips", print_winners, winnings),
            font,
            500
        )?;
        println!("tukaj sem3");
        continue_button.draw_button(canvas, font)?;
        canvas.present();
        if continue_button.is_clicked {
            return Ok(());
        }
        ::std::thread::sleep(Duration::from_millis(33));
    }
}
