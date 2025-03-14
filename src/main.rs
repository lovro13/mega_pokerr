use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::{Texture, WindowCanvas};
use std::time::Duration;
use sdl2::image::{self, InitFlag, LoadTexture};
mod card;

const CARD_HEIGHT: u32 = 120;
const CARD_WIDTH: u32 = 95;
const SCREEN_HEIGHT: u32 = 900;
const SCREEN_WIDTH: u32 = 1800;


fn render(canvas: &mut WindowCanvas, 
    background_color: Color, 
    players_list: &Vec<card::Player>
) -> Result<(), String> {

    canvas.set_draw_color(background_color);
    canvas.clear();

    let (width, height) = canvas.output_size()?;

    for player in players_list {

        
        let texture_creator = canvas.texture_creator();
        let filename = card::Card::card_to_file(&player.card);
        
        let texture = 
        match player.card_state {
            card::CardState::Closed => texture_creator.load_texture("assets/card_back.png")?,
            card::CardState::Opened => texture_creator.load_texture(filename)?
        };
        let position = player.card_position;
        let screen_position = 
        Point::new(position.0, -position.1) + Point::new(width as i32 / 2, height as i32 / 2);
        let screen_rect_card1 = Rect::from_center(screen_position, CARD_WIDTH, CARD_HEIGHT);
        let screen_position2 = screen_position + Point::new(CARD_WIDTH as i32 - 30, 0);
        let screen_rect_card2 = Rect::from_center(screen_position2, CARD_WIDTH, CARD_HEIGHT);
        canvas.copy(&texture, None, screen_rect_card1)?;
        canvas.copy(&texture, None, screen_rect_card2)?;
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
    
    // let position = Point::new(PLAYER1_CARDS.0, PLAYER1_CARDS.1);
    let mut player_list = card::Player::init_players();
    let mut event_pump = sdl_context.event_pump().unwrap();

    canvas.clear();
    canvas.present();
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
        

        render(&mut canvas, Color::RGB(200, 200, 255), &player_list)?;
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30))
    }

    Ok(())    
}
