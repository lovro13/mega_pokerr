use sdl2::{
    rect::Point,
    render::{Canvas, WindowCanvas},
    video::Window,
};

use crate::logic::{card::Card, player::Id};
use crate::sdl2_app::constants::*;

use super::render_screen::get_screen_center;

// vse točke naj bojo sredina tistega okrog česa se potem naredi pravoktnik, kar ni nujno najbolše
// glede na screen_center kot izhodišče

// Start screen positions
pub const START_BUTTON: (i32, i32) = (0, -50);
pub const SETTINGS_BUTTON_START: (i32, i32) = (0, 75);
pub const SETTINGS_BUTTON_GAME_SIZE: u32 = 70; // it was meant to be a square
pub const EXIT_BUTTON: (i32, i32) = (0, 200);
pub const TITLE_POS: (i32, i32) = (0, -200);

// Card positions on Start Screen
pub const RIGHT_CARD: (i32, i32) = (500, 0);
pub const LEFT_CARD: (i32, i32) = (-500, 0);
pub const CARD: Card = Card {
    number: crate::logic::card::CardNumber::NA,
    color: crate::logic::card::CardColor::Hearts,
};
pub const ANGLE: f64 = 20.;

// Player card positions
pub const PLAYER1_CARDS: (i32, i32) = (-50, -300);
pub const PLAYER2_CARDS: (i32, i32) = (-500, -300);
pub const PLAYER3_CARDS: (i32, i32) = (-775, 50);
pub const PLAYER4_CARDS: (i32, i32) = (-500, 325);
pub const PLAYER5_CARDS: (i32, i32) = (-50, 325);
pub const PLAYER6_CARDS: (i32, i32) = (500, 325);
pub const PLAYER7_CARDS: (i32, i32) = (700, 50);
pub const PLAYER8_CARDS: (i32, i32) = (500, -300);

// Button positions
pub const CALL_BUTTON: (i32, i32) = (0, 440);
pub const BUTTON_END_OF_ROUND: (i32, i32) = (0, 125);
pub const SETTINGS_BUTTON_GAME: (i32, i32) = (0, 125);

// Pot position
pub const POT_POSITION: (u32, u32) = (0, 100);

pub struct PlayerScreenPosition {
    // to se da še zelo nadgraditi, npr z velikostmi, ...
    pub cards: Point,
    pub name: Point,
    pub balance: Point,
}

impl Id {
    pub fn get_card_position(&self) -> (i32, i32) {
        match self {
            Id::Player1 => PLAYER1_CARDS,
            Id::Player2 => PLAYER2_CARDS,
            Id::Player3 => PLAYER3_CARDS,
            Id::Player4 => PLAYER4_CARDS,
            Id::Player5 => PLAYER5_CARDS,
            Id::Player6 => PLAYER6_CARDS,
            Id::Player7 => PLAYER7_CARDS,
            Id::Player8 => PLAYER8_CARDS,
        }
    }

    pub fn get_card_position_for_count(&self, player_count: usize) -> (i32, i32) {
        match player_count {
            2 => {
                match self {
                    Id::Player1 => (-300, -200),
                    Id::Player2 => (300, -200),
                    _ => self.get_card_position(),
                }
            }
            3 => {
                match self {
                    Id::Player1 => (-400, -200),
                    Id::Player2 => (0, -200),
                    Id::Player3 => (400, -200),
                    _ => self.get_card_position(),
                }
            }
            4 => {
                match self {
                    Id::Player1 => (-400, -200),
                    Id::Player2 => (400, -200),
                    Id::Player3 => (-400, 200),
                    Id::Player4 => (400, 200),
                    _ => self.get_card_position(),
                }
            }
            5 => {
                match self {
                    Id::Player1 => (-400, -200),
                    Id::Player2 => (0, -200),
                    Id::Player3 => (400, -200),
                    Id::Player4 => (-200, 200),
                    Id::Player5 => (200, 200),
                    _ => self.get_card_position(),
                }
            }
            6 => {
                match self {
                    Id::Player1 => (-400, -200),
                    Id::Player2 => (0, -200),
                    Id::Player3 => (400, -200),
                    Id::Player4 => (-400, 200),
                    Id::Player5 => (0, 200),
                    Id::Player6 => (400, 200),
                    _ => self.get_card_position(),
                }
            }
            7 => {
                match self {
                    Id::Player1 => (-400, -200),
                    Id::Player2 => (0, -200),
                    Id::Player3 => (400, -200),
                    Id::Player4 => (-400, 200),
                    Id::Player5 => (0, 200),
                    Id::Player6 => (400, 200),
                    Id::Player7 => (0, 0),
                    _ => self.get_card_position(),
                }
            }
            8 => self.get_card_position(),
            _ => self.get_card_position(),
        }
    }

    pub fn get_player_screen_center(&self, canvas: &WindowCanvas) -> Point {
        // is on the middle of the first card
        let (width, height) = canvas.output_size().unwrap();
        let screen_center = Point::new(width as i32 / 2, (height as i32) / 2 + SCREEN_CENTER_Y_OFFSET);
        let card_position = self.get_card_position();
        let player_center = // is on the middle of the first card
        Point::new(card_position.0, -card_position.1) + screen_center;
        return player_center;
    }

    pub fn get_player_screen_center_for_count(&self, canvas: &WindowCanvas, player_count: usize) -> Point {
        // is on the middle of the first card
        let (width, height) = canvas.output_size().unwrap();
        let screen_center = Point::new(width as i32 / 2, (height as i32) / 2 + SCREEN_CENTER_Y_OFFSET);
        let card_position = self.get_card_position_for_count(player_count);
        let player_center = // is on the middle of the first card
        Point::new(card_position.0, -card_position.1) + screen_center;
        return player_center;
    }
}

impl PlayerScreenPosition {
    pub fn new(id: Id) -> Self {
        let card_center = Point::from(Id::get_card_position(&id));
        PlayerScreenPosition {
            cards: card_center,
            name: card_center + Point::new(0, CARD_HEIGHT as i32 / 2 + 10),
            balance: card_center,
        }
    }
}

pub struct ControlPosition {
    // mogoče fino dat še velikosti
    // ali pa itak velikosti določim globalno
    pub fold_button: Point,
    pub call_button: Point,
    pub raise_button: Point,
    pub slider: Point,
}

impl ControlPosition {
    pub fn init_control_positon(canvas: &Canvas<Window>) -> ControlPosition {
        let screen_center = get_screen_center(canvas);
        let call_position = screen_center + Point::from(CALL_BUTTON);
        let fold_position = call_position - Point::new(BUTTON_WIDTH as i32 + BUTTON_SPACE as i32, 0);
        let raise_posisition =
            call_position + Point::new(BUTTON_WIDTH as i32 + BUTTON_SPACE as i32, 0);
        let slider_position =
            raise_posisition + Point::new(BUTTON_WIDTH as i32 + BUTTON_SPACE as i32, 0);
        ControlPosition {
            fold_button: fold_position,
            call_button: call_position,
            raise_button: raise_posisition,
            slider: slider_position,
        }
    }
}
