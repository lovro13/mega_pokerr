use crate::logic::player;
use sdl2::event::Event;
use sdl2::mouse::MouseButton;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::WindowCanvas;

use crate::sdl2_app::render_text::draw_text;

pub struct Button {
    pub rect: Rect,
    pub text: String,
    pub is_clicked: bool,
}

impl Button {
    pub fn new(center: &Point, height: u32, width: u32, text: String) -> Self {
        Button {
            rect: Rect::from_center(*center, width, height),
            text,
            is_clicked: false,
        }
    }

    pub fn is_hovered(&self, mouse_x: i32, mouse_y: i32) -> bool {
        self.rect.contains_point(Point::new(mouse_x, mouse_y))
    }

    pub fn draw_button(
        &self,
        canvas: &mut sdl2::render::WindowCanvas,
        font: &sdl2::ttf::Font,
    ) {
        let color = if self.is_clicked {
            Color::RGB(100, 100, 100)
        } else {
            Color::RGB(200, 200, 200)
        };

        let text_color = Color::RGB(0, 0, 0);

        canvas.set_draw_color(color);
        canvas.fill_rect(self.rect);
        draw_text(canvas, &self.text, &self.rect, font, text_color);
    }

    pub fn handle_button_events(event: &Event, button: &mut Button) {
        match event {
            Event::MouseButtonDown {
                mouse_btn: MouseButton::Left,
                x,
                y,
                ..
            } => {
                if button.is_hovered(*x, *y) {
                    button.is_clicked = true;
                    println!("Gumb je bil kliknjen!");
                }
            }
            Event::MouseButtonUp {
                mouse_btn: MouseButton::Left,
                ..
            } => {
                button.is_clicked = false;
            }
            _ => {}
        }
    }

    // soon this will make a list of all buttons [FOLD; CALL; RAISE]
    pub fn init_fold_button(canvas: &mut WindowCanvas) -> Self {
        let (width, heigth) = canvas.output_size().unwrap();
        let screen_center = Point::new((width as i32) / 2, (heigth as i32) / 2 + 100);
        let button_position = screen_center
            + Point::new(
                player::Player::PLAYER1_CARDS.0,
                -player::Player::PLAYER1_CARDS.1,
            )
            + Point::new(0, 0);
        let button_target = Rect::from_center(button_position, 100, 50);
        Button {
            rect: button_target,
            text: String::from("FOLD"),
            is_clicked: false,
        }
    }
    pub fn init_call_button(canvas: &mut WindowCanvas) -> Self {
        let (width, heigth) = canvas.output_size().unwrap();
        let screen_center = Point::new((width as i32) / 2, (heigth as i32) / 2 + 100);
        let button_position = screen_center
            + Point::new(
                player::Player::PLAYER1_CARDS.0,
                -player::Player::PLAYER1_CARDS.1,
            )
            + Point::new(110, 0);
        let button_target = Rect::from_center(button_position, 100, 50);
        Button {
            rect: button_target,
            text: String::from("CALL"),
            is_clicked: false,
        }
        
    }
    pub fn init_raise_button(canvas: &mut WindowCanvas) -> Self {
        let (width, heigth) = canvas.output_size().unwrap();
        let screen_center = Point::new((width as i32) / 2, (heigth as i32) / 2 + 100);
        let button_position = screen_center
            + Point::new(
                player::Player::PLAYER1_CARDS.0,
                -player::Player::PLAYER1_CARDS.1,
            )
            + Point::new(220, 0);
        let button_target = Rect::from_center(button_position, 100, 50);
        Button {
            rect: button_target,
            text: String::from("RAISE"),
            is_clicked: false,
        }
        
    }
}
