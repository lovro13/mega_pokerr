use mega_pokerr::logic::constants::SHOULD_RETURN_TO_START;
use mega_pokerr::logic::round::next_turn;
use mega_pokerr::sdl2_app::app_context::init_app_context;
use mega_pokerr::sdl2_app::betting_state::run_betting_state;
use mega_pokerr::sdl2_app::constants::DEBUG;
use mega_pokerr::sdl2_app::constants::MAIN_PLAYER;
use mega_pokerr::sdl2_app::end_round_state::end_round;
use mega_pokerr::sdl2_app::menu::GameSettings;
use std::sync::atomic::Ordering;

use mega_pokerr::logic::constants::SHOULD_QUIT;
use mega_pokerr::logic::game;
use mega_pokerr::logic::player;
use mega_pokerr::logic::round::begin_round;
use mega_pokerr::sdl2_app::menu::new_game_start_screen_state;
use mega_pokerr::sdl2_app::start_screen::{start_screen_state, StartScreenAction};
use std::rc::Rc;
use std::cell::RefCell;
use mega_pokerr::logic::choose_winner;
use mega_pokerr::sdl2_app::button::Button;

fn main() -> Result<(), String> {
    env_logger::init();
    log::info!("Starting Mega Poker SDL2 app");
    let app_context = init_app_context()?;
    // dobiš platno !! POMEMBNO, canvas.set_color(); canvas.clear() - zaslon v eno bravo
    // canvas.copy(...), texture -> riše slike, ali tekst
    // canvas.present() ... predstavi spremembe, ki so jih nardil .copy(), .clear()

    let mut settings = GameSettings::default();
    let mut event_pump = app_context.sdl_context.event_pump().unwrap(); // zazna inpute
    let mut canvas = app_context.canvas;

    // GLAVNA ZANKA
    // 'mainloop: loop {
    loop {
        SHOULD_RETURN_TO_START.store(false, Ordering::Relaxed);
        let game = match start_screen_state(&mut canvas, &mut event_pump, &app_context.ttf_context)?
        {
            StartScreenAction::Exit => break Ok(()),
            StartScreenAction::StartNewGame => {
                let _ = new_game_start_screen_state(
                    &mut canvas,
                    &mut event_pump,
                    &app_context.ttf_context,
                    &mut settings,
                );
                let player_list = player::Player::init_players_with_count(settings.player_count);
                let game = game::init_game(player_list);
                println!("{:#?}", game.borrow().players);
                game
            }
            StartScreenAction::LoadGame(game) => {
                log::info!("Starting game with {} players", settings.player_count);
                // Ustvari igralce z nastavitvami
                log::info!("Game initialized successfully");
                let game = Rc::new(RefCell::new(game));
                settings.player_count = game.borrow().player_count;
                game
            }
        };

        log::debug!("starting to play with game, that has players: {:#?}", game.borrow().players);

        loop {
            {
                let mut mut_game = game.borrow_mut();
                log::debug!("player count before begin_round: {}", settings.player_count);
                begin_round(&mut mut_game, settings.player_count);
                log::info!("Current street {:?}", mut_game.street);
            }
            let debug = DEBUG.load(Ordering::Relaxed);
            {
                let mut mut_game = game.borrow_mut();
                if debug {
                    log::debug!("Debug mode: showing all cards");
                    for player in mut_game.players.iter_mut() {
                        player.opened_cards = true;
                    }
                } else {
                    log::debug!("Normal mode: showing only main player cards");
                    mut_game.get_player_from_name(MAIN_PLAYER).opened_cards = true
                }
            }
            for round_num in 0..4 {
                log::debug!("Starting betting round {}", round_num + 1);
                if SHOULD_QUIT.load(Ordering::Relaxed) {
                    log::info!("Quit signal received, stopping game");
                    break;
                } else if SHOULD_RETURN_TO_START.load(Ordering::Relaxed) {
                    continue;
                }
                {
                    run_betting_state(
                        &mut canvas,
                        &mut event_pump,
                        &game,
                        &app_context.ttf_context,
                        settings.player_count,
                    )?;
                    let mut mut_game = game.borrow_mut();
                    next_turn(&mut mut_game);
                    log::debug!(
                        "Completed betting round {}, moving to next turn",
                        round_num + 1
                    );
                }
                let mut count_playing_players = 0;
                for player in game.borrow().players.iter() {
                    if player.playing {
                        count_playing_players += 1;
                    }
                }
                log::debug!("{} players still playing", count_playing_players);

                if count_playing_players <= 1 {
                    log::info!(
                        "Only {} player(s) remaining, ending round",
                        count_playing_players
                    );
                    break;
                }

                if SHOULD_QUIT.load(Ordering::Relaxed) {
                    log::info!("Stopped app at the end of main sdl2_app");
                    return Ok(());
                } else if SHOULD_RETURN_TO_START.load(Ordering::Relaxed) {
                    continue;
                }
            }
            if SHOULD_QUIT.load(Ordering::Relaxed) {
                log::info!("Quit signal received, stopping game");
                break;
            }
            if SHOULD_RETURN_TO_START.load(Ordering::Relaxed) {
                log::info!("Returning to main menu");
                break;
            }
            {
                let mut mut_game = game.borrow_mut();
                log::debug!("Showing all cards for showdown");
                for player in mut_game.players.iter_mut() {
                    player.opened_cards = true;
                }
            }
            log::info!("Starting end round sequence");
            end_round(
                &mut game.borrow_mut(),
                &mut event_pump,
                &mut canvas,
                &app_context.ttf_context,
                settings.player_count,
            )?;
            log::info!("End round completed");
        }

        if SHOULD_QUIT.load(Ordering::Relaxed) {
            log::info!("Stopped app at the end of main sdl2_app");
            return Ok(());
        } else if SHOULD_RETURN_TO_START.load(Ordering::Relaxed) {
            continue;
        } else {
            return Ok(());
        }
    }
}
