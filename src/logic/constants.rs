// mogoče fletn nastavt na začetku iger ne da so konstante
use std::sync::atomic::AtomicBool;
pub const BUY_IN: u32 = 1000;
pub const SMALL_BLIND: u32 = 10;
pub const BIG_BLIND: u32 = 20;

pub static SHOULD_QUIT: AtomicBool = AtomicBool::new(false);
