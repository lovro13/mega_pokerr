use card::PlayerPosition;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;
use std::time::Duration;
use sdl2::image::{self, InitFlag};

mod card;

const SCREEN_HEIGHT: u32 = 900;
const SCREEN_WIDTH: u32 = 1800;

pub enum Streets {
    PreFlop,
    Flop,
    Turn,
    River,
    Showdown
}

pub enum GameState {
    Paused,
    Played(card::Player)
}

fn next_player_position(player_position: &PlayerPosition) -> PlayerPosition {
    match player_position {
        PlayerPosition::Dealer => PlayerPosition::SmallBlind,
        PlayerPosition::SmallBlind => PlayerPosition::BigBlind,
        PlayerPosition::BigBlind => PlayerPosition::UnderTheGun,
        PlayerPosition::UnderTheGun => PlayerPosition::UnderTheGun1,
        PlayerPosition::UnderTheGun1 => PlayerPosition::MiddlePosition,
        PlayerPosition::MiddlePosition => PlayerPosition::Hijack,
        PlayerPosition::Hijack => PlayerPosition::Cutoff,
        PlayerPosition::Cutoff => PlayerPosition::Dealer,
        PlayerPosition::NotPlaying => PlayerPosition::NotPlaying 
    }
}

fn begin_round(player_list: &mut Vec<card::Player>) {
    let _curr_street = Streets::PreFlop;
    let deck = card::Card::make_ordered_deck();
    let mut deck = card::Card::scramble_deck(deck);


    for player in player_list {
        player.position = next_player_position(&player.position);
        let card1 = match deck.pop() {
            None => card::Card {color: card::CardColor::Empty, number: card::CardNumber::Empty}, 
            Some(card) => card
    };
    let card2 = match deck.pop() {
        None => card::Card {color: card::CardColor::Empty, number: card::CardNumber::Empty}, 
        Some(card) => card
    };
        player.cards = (card1, card2)
    }

}

fn render(canvas: &mut WindowCanvas, 
    background_color: Color, 
    players_list: &Vec<card::Player>,
    font: &sdl2::ttf::Font
) -> Result<(), String> {
    
    canvas.set_draw_color(background_color);
    canvas.clear();

    for player in players_list {
        //naprinta ime in karte igralca
        let _ = card::Player::render_player_info(canvas, player, font);
    }
    canvas.present();
    Ok(())
}

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;
    let mut font = ttf_context.load_font("font/Poppins-Black.ttf", 120).unwrap();
    font.set_style(sdl2::ttf::FontStyle::NORMAL);

    let _image_context = image::init(InitFlag::PNG | InitFlag::JPG)?;

    let window = video_subsystem.window("POKEEEER", SCREEN_WIDTH, SCREEN_HEIGHT)
    .position_centered()
    .opengl()
    .build()
    .expect("could not initialize video subsystem");

    let mut canvas = window
    .into_canvas()
    .build()
    .expect("could not make canvas");
    
    // let position = Point::new(PLAYER1_CARDS.0, PLAYER1_CARDS.1);
    let mut player_list = card::Player::init_players();
    let mut event_pump = sdl_context.event_pump().unwrap();

    canvas.clear();
    canvas.present();
    begin_round(&mut player_list);

    'running: loop {
        for event in event_pump.poll_iter(){
            match event {
                | Event::Quit { .. }
                | Event::KeyDown {keycode: Some(Keycode::Escape), ..} => {
                    break 'running;
                },
                Event::KeyDown {keycode: Some(Keycode::D), .. } => begin_round(&mut player_list),
                _ => {}
            }
        }
        

        
        render(&mut canvas, Color::RGB(200, 200, 255), &player_list, &font)?;
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30))
    }


    Ok(())    
}
