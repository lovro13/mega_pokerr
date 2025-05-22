use crate::logic::card;
use crate::logic::constants::BUY_IN;

#[derive(Debug, Clone, PartialEq)]
pub enum Id {
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
    // uu to bo treba še popravt, če nočem da je edina možna stvar 8 playerjov, ampak tud druge številke
    pub fn next_player_position(&self) -> PlayerPosition {
        match self {
            PlayerPosition::Dealer => PlayerPosition::Cutoff,
            PlayerPosition::Cutoff => PlayerPosition::Hijack,
            PlayerPosition::Hijack => PlayerPosition::MiddlePosition,
            PlayerPosition::MiddlePosition => PlayerPosition::UnderTheGun1,
            PlayerPosition::UnderTheGun1 => PlayerPosition::UnderTheGun,
            PlayerPosition::UnderTheGun => PlayerPosition::BigBlind,
            PlayerPosition::BigBlind => PlayerPosition::SmallBlind,
            PlayerPosition::SmallBlind => PlayerPosition::Dealer,
            PlayerPosition::NotPlaying => {
                panic!("NotPlaying position should not be evaluated (next_player_position)")
            }
        }
    }

    pub fn next_player_on_turn(&self) -> PlayerPosition {
        match self {
            PlayerPosition::Dealer => PlayerPosition::SmallBlind,
            PlayerPosition::SmallBlind => PlayerPosition::BigBlind,
            PlayerPosition::BigBlind => PlayerPosition::UnderTheGun,
            PlayerPosition::UnderTheGun => PlayerPosition::UnderTheGun1,
            PlayerPosition::UnderTheGun1 => PlayerPosition::MiddlePosition,
            PlayerPosition::MiddlePosition => PlayerPosition::Hijack,
            PlayerPosition::Hijack => PlayerPosition::Cutoff,
            PlayerPosition::Cutoff => PlayerPosition::Dealer,
            PlayerPosition::NotPlaying => {
                panic!("NotPlaying position should not be evaluated (next_player_on_turn)")
            }
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
            PlayerPosition::NotPlaying => {
                panic!("NotPlaying position should not be evaluated to int")
            }
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

#[derive(Debug, Clone)] // PAZIII CLONE SAMO RISANJE PLAYERJEV
pub struct Player {
    pub id: Id, // from Player1, ...,  Player8
    pub hand_cards: (card::Card, card::Card),
    pub position: PlayerPosition,
    pub chips: u32,
    pub playing: bool,
    pub current_bet: u32,
    pub debt: u32,
    pub opened_cards: bool,
}

impl Id {
    pub fn all_names() -> Vec<Id> {
        vec![
            Id::Player1,
            Id::Player2,
            Id::Player3,
            Id::Player4,
            Id::Player5,
            Id::Player6,
            Id::Player7,
            Id::Player8,
        ]
    }
}

impl Player {
    pub fn init_players() -> Vec<Player> {
        let mut list_of_players = Vec::new();
        let mut last_position = PlayerPosition::Dealer;
        let names = Id::all_names(); // 1, 2, ... , 8

        for name in names {
            let curr_position = PlayerPosition::next_player_on_turn(&last_position);
            last_position = curr_position.clone();
            let curr_player = Player {
                id: name,
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
                chips: BUY_IN,
                playing: true,
                current_bet: 0,
                debt: 0,
                opened_cards: false,
            };
            list_of_players.push(curr_player);
        }
        list_of_players
    }

    pub fn player_id_to_str(player: &Player) -> String {
        match player.id {
            Id::Player1 => String::from("Player1"),
            Id::Player2 => String::from("Player2"),
            Id::Player3 => String::from("Player3"),
            Id::Player4 => String::from("Player4"),
            Id::Player5 => String::from("Player5"),
            Id::Player6 => String::from("Player6"),
            Id::Player7 => String::from("Player7"),
            Id::Player8 => String::from("Player8"),
        }
    }
}
