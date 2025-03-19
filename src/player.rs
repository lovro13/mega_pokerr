use crate::card;
use sdl2::image::LoadTexture;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::WindowCanvas;

pub enum Names {
    Player1,
    Player2,
    Player3,
    Player4,
    Player5,
    Player6,
    Player7,
    Player8,
}

#[derive(PartialEq, Debug, Clone)]
pub enum PlayerPosition {
    Dealer,
    SmallBlind,
    BigBlind,
    UnderTheGun,
    UnderTheGun1,
    MiddlePosition,
    Hijack,
    Cutoff,
    NotPlaying,
}

impl PlayerPosition {
    pub fn next_player_position(player_position: &PlayerPosition) -> PlayerPosition {
        match player_position {
            PlayerPosition::Dealer => PlayerPosition::Cutoff,
            PlayerPosition::SmallBlind => PlayerPosition::Dealer,
            PlayerPosition::BigBlind => PlayerPosition::SmallBlind,
            PlayerPosition::UnderTheGun => PlayerPosition::BigBlind,
            PlayerPosition::UnderTheGun1 => PlayerPosition::UnderTheGun,
            PlayerPosition::MiddlePosition => PlayerPosition::UnderTheGun1,
            PlayerPosition::Hijack => PlayerPosition::MiddlePosition,
            PlayerPosition::Cutoff => PlayerPosition::Hijack,
            PlayerPosition::NotPlaying => PlayerPosition::NotPlaying,
        }
    }
}
pub struct Player {
    pub name: Names,
    pub cards: (card::Card, card::Card),
    pub card_position: (i32, i32),
    pub card_state: card::CardState,
    pub position: PlayerPosition,
    pub money: u32, // later to be finished
    pub his_turn: bool
}

impl Names {
    pub fn all_names() -> Vec<Names> {
        vec![
            Names::Player1,
            Names::Player2,
            Names::Player3,
            Names::Player4,
            Names::Player5,
            Names::Player6,
            Names::Player7,
            Names::Player8,
        ]
    }
}

impl Player {
    const PLAYER1_CARDS: (i32, i32) = (-50, -300);
    const PLAYER2_CARDS: (i32, i32) = (-500, -300);
    const PLAYER3_CARDS: (i32, i32) = (-775, 0);
    const PLAYER4_CARDS: (i32, i32) = (-500, 275);
    const PLAYER5_CARDS: (i32, i32) = (-50, 275);
    const PLAYER6_CARDS: (i32, i32) = (500, 275);
    const PLAYER7_CARDS: (i32, i32) = (700, 0);
    const PLAYER8_CARDS: (i32, i32) = (500, -300);

    pub fn init_players() -> Vec<Player> {
        let mut list_of_players = Vec::new();
        let mut last_position = PlayerPosition::Dealer;
        let mut names = Names::all_names();
        names.reverse(); // reverse da igra poteka v smeri urinega kazalca
        for name in names {
            let curr_position = PlayerPosition::next_player_position(&last_position);
            last_position = curr_position.clone();
            let curr_player = Player {
                card_position: Self::get_card_position(&name),
                name,
                cards: (
                    card::Card {
                        color: card::CardColor::Empty,
                        number: card::CardNumber::Empty,
                    },
                    card::Card {
                        color: card::CardColor::Empty,
                        number: card::CardNumber::Empty,
                    },
                ),
                card_state: card::CardState::Opened,
                position: curr_position,
                money: 1000,
                his_turn: false
            };
            list_of_players.push(curr_player);
        }
        list_of_players
    }

    pub fn get_card_position(name: &Names) -> (i32, i32) {
        match name {
            Names::Player1 => Self::PLAYER1_CARDS,
            Names::Player2 => Self::PLAYER2_CARDS,
            Names::Player3 => Self::PLAYER3_CARDS,
            Names::Player4 => Self::PLAYER4_CARDS,
            Names::Player5 => Self::PLAYER5_CARDS,
            Names::Player6 => Self::PLAYER6_CARDS,
            Names::Player7 => Self::PLAYER7_CARDS,
            Names::Player8 => Self::PLAYER8_CARDS,
        }
    }

    pub fn get_player_name(player: &Player) -> String {
        match player.name {
            Names::Player1 => String::from("Player1"),
            Names::Player2 => String::from("Player2"),
            Names::Player3 => String::from("Player3"),
            Names::Player4 => String::from("Player4"),
            Names::Player5 => String::from("Player5"),
            Names::Player6 => String::from("Player6"),
            Names::Player7 => String::from("Player7"),
            Names::Player8 => String::from("Player8"),
        }
    }

    pub fn render_player_info(
        canvas: &mut WindowCanvas,
        player: &Player,
        font: &sdl2::ttf::Font,
    ) -> Result<(), String> {
        let texture_creator = canvas.texture_creator();
        let filename1 = card::Card::card_to_file(&player.cards.0);
        let filename2 = card::Card::card_to_file(&player.cards.1);
        let (width, height) = canvas.output_size()?;

        // draw cards for player
        let (texture1, texture2) = match player.card_state {
            card::CardState::Closed => (
                texture_creator.load_texture("assets/card_back.png")?,
                texture_creator.load_texture("assets/card_back.png")?,
            ),
            card::CardState::Opened => (
                texture_creator.load_texture(filename1)?,
                texture_creator.load_texture(filename2)?,
            ),
        };
        let position = player.card_position;
        let screen_position = Point::new(position.0, -position.1)
            + Point::new(width as i32 / 2, height as i32 / 2 - 100);
        let screen_rect_card1 = Rect::from_center(screen_position, card::CARD_WIDTH, card::CARD_HEIGHT);
        let screen_position2 = screen_position + Point::new(card::CARD_WIDTH as i32 - 30, 0);
        let screen_rect_card2 = Rect::from_center(screen_position2, card::CARD_WIDTH, card::CARD_HEIGHT);
        canvas.copy(&texture1, None, screen_rect_card1)?;
        canvas.copy(&texture2, None, screen_rect_card2)?;

        // write player name near the player cards
        let text_color = Color::RGB(0, 0, 0);
        let name_text = Self::get_player_name(player);
        let name_surface = font
            .render(&name_text)
            .blended(text_color)
            .map_err(|e| e.to_string())?;

        let text_texture = texture_creator
            .create_texture_from_surface(&name_surface)
            .map_err(|e| e.to_string())?;

        let screen_position3 = screen_position + Point::new(30, 70);
        let text_target = Rect::from_center(screen_position3, 150 as u32, 75 as u32);
        canvas.copy(&text_texture, None, Some(text_target))?;

        let balance_color = Color::RGB(0, 0, 10);
        let balance_text = format!("Balance: {}", player.money);
        let balance_surface = font
            .render(&balance_text)
            .blended(balance_color)
            .map_err(|e| e.to_string())?;

        let text_texture = texture_creator
            .create_texture_from_surface(&balance_surface)
            .map_err(|e| e.to_string())?;

        let screen_position3 = screen_position + Point::new(30, 120);
        let text_target = Rect::from_center(screen_position3, 150 as u32, 75 as u32);
        canvas.copy(&text_texture, None, Some(text_target))?;

        if player.position == PlayerPosition::Dealer {
            let texture = texture_creator.load_texture("assets/dealer_token.png")?;
            let screen_position4 = screen_position + Point::new(150, 100);
            let screen_rect_dealer = Rect::from_center(screen_position4, 70, 70);
            canvas.copy(&texture, None, screen_rect_dealer)?;
        }

        Ok(())
    }
}
