// mogoče fletn nastavt na začetku iger ne da so konstante
use std::sync::atomic::AtomicBool;

// Game rules and betting constants
pub const BUY_IN: u32 = 1000;
pub const SMALL_BLIND: u32 = 10;
pub const BIG_BLIND: u32 = 20;

// Bot strategy constants
pub const BOT_RANK_THRESHOLD_LOW: u32 = 30;
pub const BOT_RANK_THRESHOLD_MEDIUM: u32 = 45;
pub const BOT_RANK_THRESHOLD_HIGH: u32 = 50;
pub const BOT_BLIND_MULTIPLIER_LOW: u32 = 2;
pub const BOT_BLIND_MULTIPLIER_MEDIUM: u32 = 5;

// Card evaluation constants
pub const CARD_VALUES_COUNT: usize = 13;
pub const CARD_COLORS_COUNT: usize = 4;
pub const CARDS_IN_DECK: usize = 52;
pub const CARDS_IN_HAND: usize = 2;
pub const CARDS_ON_TABLE_MAX: usize = 5;

// Hand ranking values
pub const ROYAL_FLUSH_RANK: i32 = 10;
pub const STRAIGHT_FLUSH_RANK: i32 = 9;
pub const FOUR_OF_A_KIND_RANK: i32 = 8;
pub const FULL_HOUSE_RANK: i32 = 7;
pub const FLUSH_RANK: i32 = 6;
pub const STRAIGHT_RANK: i32 = 5;
pub const THREE_OF_A_KIND_RANK: i32 = 4;
pub const TWO_PAIR_RANK: i32 = 3;
pub const ONE_PAIR_RANK: i32 = 2;
pub const HIGH_CARD_RANK: i32 = 1;

// Card number values
pub const ACE_VALUE: u32 = 14;
pub const KING_VALUE: u32 = 13;
pub const QUEEN_VALUE: u32 = 12;
pub const JACK_VALUE: u32 = 11;
pub const TEN_VALUE: u32 = 10;
pub const TWO_VALUE: u32 = 2;

// Game state constants - konfigurabilno število igralcev
pub const DEFAULT_PLAYER_COUNT: usize = 6; // Spremenjeno iz 8 na 6 kot privzeto
pub const MAX_PLAYERS: usize = 8;
pub const MIN_PLAYERS: usize = 2;

// Threading and control constants
pub static SHOULD_QUIT: AtomicBool = AtomicBool::new(false);

pub static DATABASE_PATH: &str = "poker.db";
