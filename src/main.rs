extern crate sdl2;

use::sdl2::event::Event;
use::sdl2::keyboard::Keycode;
use::sdl2::pixels::Color;
use::std::time::Duration;


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


fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("POKEEEER", 800, 600)
    .position_centered()
    .opengl()
    .build()
    .unwrap();

    let mut canvas = window
    .into_canvas()
    .build()
    .unwrap();

    canvas.set_draw_color(Color::RGB(255, 0, 0));
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();

    'running: loop {
        for event in event_pump.poll_iter(){
            match event {
                | Event::Quit { .. }
                | Event::KeyDown { 
                    keycode: Some(Keycode::Escape),
                    ..
                 } => break 'running,
                _ => {}
            }
        }

        canvas.clear();
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30))
    }

    
}
