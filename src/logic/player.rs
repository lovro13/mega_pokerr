use crate::logic::card;
use crate::logic::constants::BUY_IN;

#[derive(Debug, Clone)]
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
impl Iterator for PlayerPosition {
    type Item = PlayerPosition;

    fn next(&mut self) -> Option<Self::Item> {
        let next_pos = self.next_player_position();
        Some(std::mem::replace(self, next_pos))
    }
}
impl PlayerPosition {
    pub fn next_player_position(&self) -> PlayerPosition {
        match self {
            PlayerPosition::Dealer => PlayerPosition::SmallBlind,
            PlayerPosition::SmallBlind => PlayerPosition::BigBlind,
            PlayerPosition::BigBlind => PlayerPosition::UnderTheGun,
            PlayerPosition::UnderTheGun => PlayerPosition::UnderTheGun1,
            PlayerPosition::UnderTheGun1 => PlayerPosition::MiddlePosition,
            PlayerPosition::MiddlePosition => PlayerPosition::Hijack,
            PlayerPosition::Hijack => PlayerPosition::Cutoff,
            PlayerPosition::Cutoff => PlayerPosition::Dealer,
            PlayerPosition::NotPlaying => panic!("NotPlaying position should not be evaluated (next_player_position)"),
        }
    }

    pub fn eval_to_int(&self) -> u32 {
        match self {
            PlayerPosition::Dealer => 0,
            PlayerPosition::SmallBlind => 1,
            PlayerPosition::BigBlind => 2,
            PlayerPosition::UnderTheGun => 3,
            PlayerPosition::UnderTheGun1 => 4,
            PlayerPosition::MiddlePosition => 5,
            PlayerPosition::Hijack => 6,
            PlayerPosition::Cutoff => 7,
            PlayerPosition::NotPlaying => panic!("NotPlaying position should not be evaluated to int"),
        }
    }

    pub fn eval_from_int_to_position(num: u32) -> PlayerPosition {
        match num {
            0 => PlayerPosition::Dealer,
            1 => PlayerPosition::SmallBlind,
            2 => PlayerPosition::BigBlind,
            3 => PlayerPosition::UnderTheGun,
            4 => PlayerPosition::UnderTheGun1,
            5 => PlayerPosition::MiddlePosition,
            6 => PlayerPosition::Hijack,
            7 => PlayerPosition::Cutoff,
            _ => panic!("Invalid player position"),
        }
    }
}

pub struct Player {
    pub name: Names,
    pub hand_cards: (card::Card, card::Card),
    pub card_position: (i32, i32),
    pub position: PlayerPosition,
    pub money: u32,
    pub playing: bool,
    pub current_bet: u32,
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
    pub const PLAYER1_CARDS: (i32, i32) = (-50, -300);
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
                hand_cards: (
                    card::Card {
                        color: card::CardColor::Empty,
                        number: card::CardNumber::Empty,
                    },
                    card::Card {
                        color: card::CardColor::Empty,
                        number: card::CardNumber::Empty,
                    },
                ),
                position: curr_position,
                money: BUY_IN,
                playing: true,
                current_bet: 0,
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
}
