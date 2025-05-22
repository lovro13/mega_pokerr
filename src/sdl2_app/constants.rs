use std::sync::atomic::AtomicBool;
use crate::logic::player::Id;

pub const SCREEN_HEIGHT: u32 = 900;
pub const SCREEN_WIDTH: u32 = 1800;



pub const  BACKGROUND_COLOR: (u8, u8, u8) = (200, 200, 255);
pub const INFO_B_COLOR: (u8, u8, u8) = (255, 102, 102);
pub const BUTTON_COLOR: (u8, u8, u8) = (200, 200, 200);
pub const BUTTON_COLOR_PRESSED: (u8, u8, u8) = (100, 100, 100);

pub const MAIN_PLAYER: Id = Id::Player1;

pub static DEBUG: AtomicBool = AtomicBool::new(false);

