use std::sync::atomic::Ordering;

use mega_pokerr::logic::round::next_turn;
use mega_pokerr::sdl2_app::betting_state::run_betting_state;
use mega_pokerr::sdl2_app::constants::DEBUG;
use mega_pokerr::sdl2_app::constants::MAIN_PLAYER;
use mega_pokerr::sdl2_app::end_round_state::end_round;
use mega_pokerr::sdl2_app::resources::init_app_context;

use mega_pokerr::logic::constants::SHOULD_QUIT;
use mega_pokerr::logic::game;
use mega_pokerr::logic::player;
use mega_pokerr::logic::round::begin_round;
use mega_pokerr::sdl2_app::start_screen::start_screen_state;

fn main() -> Result<(), String> {
    let app_context = init_app_context()?;
    // dobiš platno !! POMEMBNO, canvas.set_color(); canvas.clear() - zaslon v eno bravo
    // canvas.copy(...), texture -> riše slike, ali tekst
    // canvas.present() ... predstavi spremembe, ki so jih nardil .copy(), .clear()

    let player_list = player::Player::init_players();
    let game = game::init_game(player_list);

    let mut event_pump = app_context.sdl_context.event_pump().unwrap(); // zazna inpute
    let mut canvas = app_context.canvas;

    // GLAVNA ZANKA
    let start = start_screen_state(&mut canvas, &mut event_pump, &app_context.ttf_context);

    if !start? {
        return Ok(());
    }
    
    loop {
        {
            let mut mut_game = game.borrow_mut();
            begin_round(&mut mut_game);
            println!("Current street {:?}", mut_game.street);
        }
        let debug = DEBUG.load(Ordering::Relaxed);
        {
            let mut mut_game = game.borrow_mut();
            if debug {
                for player in mut_game.players.iter_mut() {
                    player.opened_cards = true;
                }
            } else {
                mut_game.get_player_from_name(MAIN_PLAYER).opened_cards = true
            }
        }
        for _ in 0..4 {
            if SHOULD_QUIT.load(Ordering::Relaxed) {
                break;
            }
            {
                run_betting_state(&mut canvas, &mut event_pump, &game, &app_context.ttf_context)?;
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
        {
            let mut mut_game = game.borrow_mut();
            for player in mut_game.players.iter_mut() {
                player.opened_cards = true;
            }
        }
        end_round(&mut game.borrow_mut(), &mut event_pump, &mut canvas, &app_context.ttf_context)?;
    }
    println!("Stopped app at the end of main sdl2_app");
    Ok(())
}
