use mega_pokerr::logic::choose_winner::choose_winner;
use mega_pokerr::logic::round::next_turn;
use mega_pokerr::sdl2_app::betting_state::run_betting_state;
use sdl2::image::{self, InitFlag};

use mega_pokerr::logic::game;
use mega_pokerr::logic::player;
use mega_pokerr::logic::round::begin_round;
use mega_pokerr::sdl2_app::constants::{SCREEN_HEIGHT, SCREEN_WIDTH};

pub enum GameState {
    Paused,
    Played(player::Player),
}

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let _image_context = image::init(InitFlag::PNG | InitFlag::JPG)?;
    let window = video_subsystem
        .window("POKEEEER", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .expect("could not initialize video subsystem");

    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;
    let mut font = ttf_context
        .load_font("font/Poppins-Black.ttf", 120)
        .unwrap();
    font.set_style(sdl2::ttf::FontStyle::NORMAL);

    let mut canvas = window.into_canvas().build().expect("could not make canvas");
    // dobiš platno !! POMEMBNO, canvas.set_color(); canvas.clear() - zaslon v eno bravo
    // canvas.copy(...), texture -> riše slike, ali tekst
    // canvas.present() ... predstavi spremembe, ki so jih nardil .copy(), .clear()

    let player_list = player::Player::init_players();
    let game = game::init_game(player_list);

    let mut event_pump = sdl_context.event_pump().unwrap();
    // zazna inpute

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
        run_betting_state(&mut canvas, &mut event_pump, &game, &font)?;
        {
            let mut mut_game = game.borrow_mut();
            next_turn(&mut mut_game);
            println!("Current street {:?}", mut_game.street);
            println!("board_cards : {:?}", mut_game.board_cards);
        }
        run_betting_state(&mut canvas, &mut event_pump, &game, &font)?;
        {
            let mut mut_game = game.borrow_mut();
            next_turn(&mut mut_game);
            println!("Current street {:?}", mut_game.street);
            println!("board_cards : {:?}", mut_game.board_cards);
        }
        run_betting_state(&mut canvas, &mut event_pump, &game, &font)?;
        {
            let mut mut_game = game.borrow_mut();
            next_turn(&mut mut_game);
            println!("board_cards : {:?}", mut_game.board_cards);
            println!("Current street {:?}", mut_game.street);
        }
        run_betting_state(&mut canvas, &mut event_pump, &game, &font)?;
        {
            let mut mut_game = game.borrow_mut();
            next_turn(&mut mut_game);
            println!("board_cards : {:?}", mut_game.board_cards);
            println!("Current street {:?}", mut_game.street);
        }
        let curr_pot = {game.borrow().pot};
        let mut mut_game = game.borrow_mut();
        let mut winners = choose_winner(&mut mut_game);
        let winnings = curr_pot - winners.len() as u32;
        for winner in winners.iter_mut() {
            winner.chips += winnings;
        }
        println!("{:#?}", winners);
    }
    println!("Stopped app at the end of main sdl2_app");
    Ok(())
}
