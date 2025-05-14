use std::sync::atomic::Ordering;

use mega_pokerr::logic::round::next_turn;
use mega_pokerr::sdl2_app::betting_state::run_betting_state;
use mega_pokerr::sdl2_app::end_round_state::end_round;
use mega_pokerr::sdl2_app::resources::init_app_context;

use mega_pokerr::logic::game;
use mega_pokerr::logic::player;
use mega_pokerr::logic::round::begin_round;
use mega_pokerr::logic::constants::SHOULD_QUIT;
pub enum GameState {
    Paused,
    Played(player::Player),
}

fn main() -> Result<(), String> {
    let app_context = init_app_context()?;
    // dobiš platno !! POMEMBNO, canvas.set_color(); canvas.clear() - zaslon v eno bravo
    // canvas.copy(...), texture -> riše slike, ali tekst
    // canvas.present() ... predstavi spremembe, ki so jih nardil .copy(), .clear()

    let player_list = player::Player::init_players();
    let game = game::init_game(player_list);

    let mut event_pump = app_context.sdl_context.event_pump().unwrap(); // zazna inpute
    let mut canvas = app_context.canvas;
    let font = app_context
        .ttf_context
        .load_font("assets/font/Poppins-Black.ttf", 120)
        .map_err(|e| e.to_string())?;

    // GLAVNA ZANKA
    loop {
        if game.borrow().round_number > 2 {
            break;
        }
        {
            let mut mut_game = game.borrow_mut();
            begin_round(&mut mut_game);
            println!("Current street {:?}", mut_game.street);
        }
        for _ in 0..4 {
            if SHOULD_QUIT.load(Ordering::Relaxed) {
                break;
            }
            {
                run_betting_state(&mut canvas, &mut event_pump, &game, &font)?;
                let mut mut_game = game.borrow_mut();
                next_turn(&mut mut_game);
            }
            let mut count_playing_players = 0;
            for player in game.borrow().players.iter() {
                if player.playing {
                    count_playing_players += 1;
                }
            }

            if count_playing_players <= 1 {
                break;
            }
        }
        if SHOULD_QUIT.load(Ordering::Relaxed) {
            break;
        }
        end_round(&mut game.borrow_mut(), &mut event_pump, &mut canvas, &font)?;
    }
    println!("Stopped app at the end of main sdl2_app");
    Ok(())
}
