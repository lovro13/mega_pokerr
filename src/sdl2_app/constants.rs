use std::sync::atomic::AtomicBool;

use crate::logic::player::Names;

pub const SCREEN_HEIGHT: u32 = 900;
pub const SCREEN_WIDTH: u32 = 1800;

pub const CARD_HEIGHT: u32 = 120;
pub const CARD_WIDTH: u32 = 95;
pub const FOLD_BUTTON: (i32, i32) = (0, 0);

pub const BACKGROUND_COLOR: (u8, u8, u8) = (200, 200, 255);
pub const MAIN_PLAYER: Names = Names::Player1;

pub static DEBUG: AtomicBool = AtomicBool::new(false);
