use::sdl2::event::Event;
use::sdl2::keyboard::Keycode;
use::sdl2::pixels::Color;
use sdl2::{rect::{Point, Rect}, render::{Texture, WindowCanvas}, sys::CapButt};
use::std::time::Duration;
use sdl2::image::{self, InitFlag, LoadTexture};

const CARD_HEIGHT: u32 = 100;
const CARD_WIDTH: u32 = 80;
const SCREEN_HEIGHT: u32 = 1000;
const SCREEN_WIDTH: u32 = 1500;

enum CardNumber {
    // označeno R kot rang karte, to sem si izmislil
    // nevem če je to izraz
    N2, N3, N4, N5, N6, N7,
    N8, N9, N10, NJ, RQ, NK, NA    
}
enum CardColor {
    Hearts, Spades, Diamonds, Clubs 
}

struct Card {
    color: CardColor,
    number: CardNumber
}

fn render(canvas: &mut WindowCanvas, 
    color: Color, 
    texture: &Texture, 
    position: Point
) -> Result<(), String> {
    canvas.set_draw_color(color);
    canvas.clear();

    let (width, height) = canvas.output_size()?;

    let screen_position = position + Point::new(width as i32 / 2, height as i32 / 2);
    let screen_rect = Rect::from_center(screen_position, CARD_WIDTH, CARD_HEIGHT);

    canvas.copy(texture, None, screen_rect)?;

    canvas.present();

    Ok(())
}

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let _image_context = image::init(InitFlag::PNG | InitFlag::JPG)?;

    let window = video_subsystem.window("POKEEEER", 1200, 600)
    .position_centered()
    .opengl()
    .build()
    .expect("could not initialize video subsystem");

    let mut canvas = window
    .into_canvas()
    .build()
    .expect("could not make canvas");

    let texture_creator = canvas.texture_creator();
    let texture = texture_creator.load_texture("assets/2_of_clubs.png")?;
    let position = Point::new(0, 0);

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
        

        i = (i + 1) % 255;
        render(&mut canvas, Color::RGB(i, 64, 255 - i), &texture, position)?;
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30))
    }

    Ok(())    
}
