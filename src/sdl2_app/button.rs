use sdl2::event::Event;
use sdl2::mouse::MouseButton;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::WindowCanvas;

use super::constants::*;
use super::positions::*;
use crate::sdl2_app::render_screen::get_screen_center;
use crate::sdl2_app::render_text::draw_text;

pub struct Button {
    pub center: Point,
    pub height: u32,
    pub width: u32,
    pub text: String,
    pub is_clicked: bool,
}

impl Button {
    pub fn new(center: Point, height: u32, width: u32, text: String) -> Self {
        Button {
            center,
            height,
            width,
            text,
            is_clicked: false,
        }
    }

    pub fn draw_button(
        &self,
        canvas: &mut sdl2::render::WindowCanvas,
        ttf_context: &sdl2::ttf::Sdl2TtfContext,
        text_size: u16,
    ) -> Result<(), String> {
        let color = if self.is_clicked {
            BUTTON_COLOR_PRESSED
        } else {
            BUTTON_COLOR
        };

        let text_color = Color::RGB(0, 0, 0);
        draw_text(
            canvas,
            &self.text,
            self.get_button_rect(),
            &ttf_context,
            text_size,
            text_color,
            Some(color),
            true,
        )?;
        Ok(())
    }

    pub fn get_button_rect(&self) -> Rect {
        let pos = self.center;
        Rect::from_center(Point::new(pos.x, pos.y), self.width, self.height)
    }

    pub fn is_hovered(&self, x: i32, y: i32) -> bool {
        let rect = self.get_button_rect();
        rect.contains_point(Point::new(x, y))
    }

    pub fn handle_button_events(event: &Event, button: &mut Button) {
        match event {
            Event::MouseButtonDown {
                mouse_btn: MouseButton::Left,
                x,
                y,
                ..
            } => {
                // println!(
                //     "debug v render_button: {}.is_hovered {}",
                //     button.text, button.is_hovered(*x, *y, canvas)
                // );
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
        Button {
            center: button_position,
            height: BUTTON_HEIGHT,
            width: BUTTON_WIDTH,
            text: String::from("FOLD"),
            is_clicked: false,
        }
    }
    pub fn init_call_button(canvas: &mut WindowCanvas) -> Self {
        let button_position = ControlPosition::init_control_positon(canvas).call_button;
        Button {
            center: button_position,
            height: BUTTON_HEIGHT,
            width: BUTTON_WIDTH,
            text: String::from("CALL"),
            is_clicked: false,
        }
    }
    pub fn init_check_button(canvas: &mut WindowCanvas) -> Self {
        let button_position = ControlPosition::init_control_positon(canvas).call_button;
        Button {
            center: button_position,
            height: BUTTON_HEIGHT,
            width: BUTTON_WIDTH,
            text: String::from("CHECK"),
            is_clicked: false,
        }
    }
    pub fn init_raise_button(canvas: &mut WindowCanvas) -> Self {
        let button_position = ControlPosition::init_control_positon(canvas).raise_button;
        Button {
            center: button_position,
            height: BUTTON_HEIGHT,
            width: BUTTON_WIDTH,
            text: String::from("RAISE"),
            is_clicked: false,
        }
    }
    pub fn init_allin_button(canvas: &mut WindowCanvas) -> Self {
        let button_position = ControlPosition::init_control_positon(canvas).raise_button;
        Button {
            center: button_position,
            height: BUTTON_HEIGHT,
            width: BUTTON_WIDTH,
            text: String::from("ALL IN"),
            is_clicked: false,
        }
    }

    pub fn init_end_of_round_button(canvas: &mut WindowCanvas) -> Self {
        let screen_center = get_screen_center(canvas);
        let button_position = screen_center + Point::from(BUTTON_END_OF_ROUND);
        Button {
            center: button_position,
            height: BUTTON_END_OF_ROUND_HEIGHT,
            width: BUTTON_END_OF_ROUND_WIDTH,
            text: String::from("START NEW ROUND"),
            is_clicked: false,
        }
    }

    pub fn init_settings_button() -> Self {
        let button_position = Point::from((SETTINGS_BUTTON_GAME.0 + SETTINGS_BUTTON_GAME_WIDTH as i32, SETTINGS_BUTTON_GAME.1 / 2 as i32));
        Button {
            center: button_position,
            height: SETTINGS_BUTTON_GAME_HEGITH,
            width: SETTINGS_BUTTON_GAME_WIDTH,
            text: String::from("Settings"),
            is_clicked: false,
        }
    }
    

    pub fn init_resume_button(canvas: &mut WindowCanvas) -> Self {
        let screen_center = get_screen_center(canvas);
        let offset = -((SETTINGS_BUTTON_HEIGTH as i32) + SETTINGS_BUTTON_SPACING);
        let button_position = screen_center + Point::new(0, offset);
        Button {
            center: button_position,
            height: SETTINGS_BUTTON_HEIGTH,
            width: SETTINGS_BUTTON_WIDTH,
            text: String::from("RESUME"),
            is_clicked: false,
        }
    }

    pub fn init_save_button(canvas: &mut WindowCanvas) -> Self {
        let screen_center = get_screen_center(canvas);
        Button {
            center: screen_center,
            height: SETTINGS_BUTTON_HEIGTH,
            width: SETTINGS_BUTTON_WIDTH,
            text: String::from("SAVE GAME"),
            is_clicked: false,
        }
    }

    pub fn init_exit_button(canvas: &mut WindowCanvas) -> Self {
        let screen_center = get_screen_center(canvas);
        let offset = (SETTINGS_BUTTON_HEIGTH as i32) + SETTINGS_BUTTON_SPACING;
        let button_position = screen_center + Point::new(0, offset);
        Button {
            center: button_position,
            height: SETTINGS_BUTTON_HEIGTH,
            width: SETTINGS_BUTTON_WIDTH,
            text: String::from("EXIT"),
            is_clicked: false,
        }
    }
}
