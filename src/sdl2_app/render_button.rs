use sdl2::event::Event;
use sdl2::mouse::MouseButton;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::WindowCanvas;

use crate::sdl2_app::render_text::draw_text;
use crate::sdl2_app::render_screen::get_screen_center;
use super::constants::{BUTTON_COLOR, BUTTON_COLOR_PRESSED};
use super::positions::{ControlPosition, BUTTON_END_OF_ROUND, BUTTON_END_OF_ROUND_HEIGHT, BUTTON_END_OF_ROUND_WIDTH};

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
    ) -> Result<(), String> {
        let color = if self.is_clicked {
            Color::RGB(BUTTON_COLOR_PRESSED.0, BUTTON_COLOR_PRESSED.1, BUTTON_COLOR_PRESSED.2)
        } else {
            Color::RGB(BUTTON_COLOR.0, BUTTON_COLOR.1, BUTTON_COLOR.2)
        };

        let text_color = Color::RGB(0, 0, 0);

        draw_text(canvas, &self.text, self.rect, font, text_color, Some(color))?;
        Ok(())
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
        let button_position = ControlPosition::init_control_positon(canvas).fold_button;
        let button_target = Rect::from_center(button_position, 100, 50);
        Button {
            rect: button_target,
            text: String::from("FOLD"),
            is_clicked: false,
        }
    }
    pub fn init_call_button(canvas: &mut WindowCanvas) -> Self {
        let button_position = ControlPosition::init_control_positon(canvas).call_button;
        let button_target = Rect::from_center(button_position, 100, 50);
        Button {
            rect: button_target,
            text: String::from("CALL"),
            is_clicked: false,
        }
    }
    pub fn init_check_button(canvas: &mut WindowCanvas) -> Self {
        let button_position = ControlPosition::init_control_positon(canvas).call_button;
        let button_target = Rect::from_center(button_position, 100, 50);
        Button {
            rect: button_target,
            text: String::from("CHECK"),
            is_clicked: false,
        }
    }
    pub fn init_raise_button(canvas: &mut WindowCanvas) -> Self {
        let button_position = ControlPosition::init_control_positon(canvas).raise_button;
        let button_target = Rect::from_center(button_position, 100, 50);
        Button {
            rect: button_target,
            text: String::from("RAISE"),
            is_clicked: false,
        }
    }
    pub fn init_allin_button(canvas: &mut WindowCanvas) -> Self {
        let button_position = ControlPosition::init_control_positon(canvas).raise_button;
        let button_target = Rect::from_center(button_position, 100, 50);
        Button {
            rect: button_target,
            text: String::from("ALL IN"),
            is_clicked: false,
        }
    }

    pub fn init_end_of_round_button(canvas: &mut WindowCanvas) -> Self {
        let screen_center = get_screen_center(canvas);
        let button_position = screen_center + Point::from(BUTTON_END_OF_ROUND);
        let button_target = Rect::from_center(button_position, BUTTON_END_OF_ROUND_WIDTH, BUTTON_END_OF_ROUND_HEIGHT);
        Button {
            rect: button_target,
            text: String::from("START NEW ROUND"),
            is_clicked: false,
        }
    }
}
