use std::sync::atomic::Ordering;
use std::time::Duration;

use sdl2::keyboard::Keycode;
use sdl2::{event::Event, EventPump};

use crate::logic::{choose_winner, game::Game, constants::SHOULD_QUIT};
use crate::sdl2_app::render_text::write_info;
use crate::sdl2_app::render_button::Button;

use super::constants::*;
use super::render_screen::render_screen;

pub fn end_round(
    game: &mut Game,
    event_pump: &mut EventPump,
    canvas: &mut sdl2::render::WindowCanvas,
    ttf_context: &sdl2::ttf::Sdl2TtfContext,
    player_count: usize,
) -> Result<(), String> {
    log::info!("Starting end round sequence");
    let curr_pot = game.pot;
    let mut winners = choose_winner::choose_winner(game);
    let winnings = curr_pot / winners.len() as u32;
    let mut print_winners = vec![];
    for winner in winners.iter_mut() {
        winner.chips += winnings;
        print_winners.push(winner.id.clone());
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
                    SHOULD_QUIT.store(true, Ordering::Relaxed);
                    return Ok(()); // Signal za izhod
                }
                _ => {}
            }
        }

        render_screen(canvas, LIGHT_BLUE, game, ttf_context, player_count)?;
        write_info(
            canvas,
            &format!("{:?} won the round and {} chips", print_winners, winnings),
            &ttf_context,
            WRITE_INFO_SIZE
        )?;
        continue_button.draw_button(canvas, &ttf_context, 20)?;
        canvas.present();
        if continue_button.is_clicked {
            return Ok(());
        }
        ::std::thread::sleep(Duration::from_millis(33));
    }
}
