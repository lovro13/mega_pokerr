use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::{Texture, WindowCanvas};
use std::time::Duration;
use sdl2::image::{self, InitFlag, LoadTexture};
mod models;

const CARD_HEIGHT: u32 = 120;
const CARD_WIDTH: u32 = 95;
const SCREEN_HEIGHT: u32 = 900;
const SCREEN_WIDTH: u32 = 1800;

const PLAYER1_CARDS: (i32, i32) = (-775, 0);
const PLAYER2_CARDS: (i32, i32) = (-500, 275);
const PLAYER3_CARDS: (i32, i32) = (700, 0);
const PLAYER4_CARDS: (i32, i32) = (500, 275);
const PLAYER5_CARDS: (i32, i32) = (-50, 275);
const PLAYER6_CARDS: (i32, i32) = (-50, -300);
const PLAYER7_CARDS: (i32, i32) = (-500, -300);
const PLAYER8_CARDS: (i32, i32) = (500, -300);

fn render(canvas: &mut WindowCanvas, 
    color: Color, 
    texture: &Texture
) -> Result<(), String> {
    let player_cards_position: Vec<(i32, i32)> = 
    vec![PLAYER1_CARDS, PLAYER2_CARDS, PLAYER3_CARDS, PLAYER4_CARDS, 
    PLAYER5_CARDS, PLAYER6_CARDS, PLAYER7_CARDS, PLAYER8_CARDS];
    canvas.set_draw_color(color);
    canvas.clear();

    let (width, height) = canvas.output_size()?;

    for position in player_cards_position {
        let screen_position = 
        Point::new(position.0, -position.1) + Point::new(width as i32 / 2, height as i32 / 2);
        let screen_rect_card1 = Rect::from_center(screen_position, CARD_WIDTH, CARD_HEIGHT);
        let screen_position2 = screen_position + Point::new(CARD_WIDTH as i32 - 30, 0);
        let screen_rect_card2 = Rect::from_center(screen_position2, CARD_WIDTH, CARD_HEIGHT);
        canvas.copy(texture, None, screen_rect_card1)?;
        canvas.copy(texture, None, screen_rect_card2)?;
    }

    canvas.present();

    Ok(())
}

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

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

    let texture_creator = canvas.texture_creator();
    let card: models::card::Card = models::card::Card {
        color: models::card::CardColor::Spades,
        number: models::card::CardNumber::N2
    };
    let filename = models::card::Card::card_to_file(card);
    let texture = texture_creator.load_texture(filename)?;
    // let position = Point::new(PLAYER1_CARDS.0, PLAYER1_CARDS.1);

    let mut event_pump = sdl_context.event_pump().unwrap();

    canvas.clear();
    canvas.present();
    let mut i = 0;
    'running: loop {
        for event in event_pump.poll_iter(){
            match event {
                | Event::Quit { .. }
                | Event::KeyDown {keycode: Some(Keycode::Escape), ..} => {
                    break 'running;
                },
                _ => {}
            }
        }
        

        render(&mut canvas, Color::RGB(200, 200, 255), &texture)?;
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30))
    }

    Ok(())    
}
