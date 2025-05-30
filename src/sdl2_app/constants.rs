use std::sync::atomic::AtomicBool;
use sdl2::pixels::Color;

use crate::logic::player::Id;

pub const SCREEN_HEIGHT: u32 = 1080;
pub const SCREEN_WIDTH: u32 = 1920;

pub const PLAYER_INFO_FONT_SIZE: u16 = 40;
pub const BUTTON_FONT_SIZE: u16 = 40;
pub const SLIDER_FONT_SIZE: u16 = 30;
pub const WRITE_INFO_SIZE: u16 = 40;

pub const BACKGROUND_COLOR: Color = Color { r: 200, g: 200, b: 255, a: 0xff };
pub const INFO_B_COLOR: Color = Color { r: 255, g: 102, b: 102, a: 0xff };
pub const BUTTON_COLOR: Color = Color { r: 200, g: 200, b: 200, a: 0xff };
pub const BUTTON_COLOR_PRESSED: Color = Color { r: 100, g: 100, b: 100, a: 0xff };
pub const BALANCE_COLOR: Color = Color { r: 0, g: 0, b: 10, a: 0xff };
pub const FOLDED_COLOR: Color = Color { r: 128, g: 128, b: 128, a: 0xff };
pub const BLACK: Color = Color { r: 0, g: 0, b: 10, a: 0xff };

pub const MAIN_PLAYER: Id = Id::Player1;

pub const PATH_TO_FONT: &str = "assets/font/Poppins-Black.ttf";

pub static DEBUG: AtomicBool = AtomicBool::new(false);

