#[derive(Clone)]
pub enum CardNumber {
    // označeno R kot rang karte, to sem si izmislil
    // nevem če je to izraz
    N2, N3, N4, N5, N6, N7,
    N8, N9, N10, NJ, NQ, NK, NA,
    Empty
}

#[derive(Clone)]
pub enum CardColor {
    Hearts, Spades, Diamonds, Clubs, Empty 
}

pub struct Card {
    pub color: CardColor,
    pub number: CardNumber
}


impl Card {
    pub fn all_colors() -> Vec<CardColor> {
        vec![CardColor::Hearts, CardColor::Spades, 
        CardColor::Diamonds, CardColor::Clubs]
    }

    pub fn all_numbers() -> Vec<CardNumber> {
        vec![CardNumber::N2, CardNumber::N3, CardNumber::N4, 
        CardNumber::N5, CardNumber::N6, CardNumber::N7,
        CardNumber::N8, CardNumber::N9, CardNumber::N10, 
        CardNumber::NJ, CardNumber::NQ, CardNumber::NK, CardNumber::NA
        ]
    }

    pub fn make_ordered_deck() -> Vec<Card> {
        let mut all = Vec::new();
        for number in Self::all_numbers() {
            for color in Self::all_colors() {
                all.push(Card {number: number.clone(), color: color.clone()})
            }
        }
        all
    }
}

impl Card {
    pub fn card_to_file(card: &Card) -> String {

        let Card { color, number } = card;
        let string2 = match color {
            CardColor::Hearts => String::from("_of_hearts.png"),
            CardColor::Spades => String::from("_of_spades.png"),
            CardColor::Diamonds => String::from("_of_diamonds.png"),
            CardColor::Clubs => String::from("_of_clubs.png"),
            CardColor::Empty => String::from("")
        };
        let string1 = match number  {
            CardNumber::N2 => String::from("2"),
            CardNumber::N3 => String::from("3"),
            CardNumber::N4 => String::from("4"),
            CardNumber::N5 => String::from("5"),
            CardNumber::N6 => String::from("6"),
            CardNumber::N7 => String::from("7"),
            CardNumber::N8 => String::from("8"),
            CardNumber::N9 => String::from("9"),
            CardNumber::N10 => String::from("10"),
            CardNumber::NJ => String::from("jack"),
            CardNumber::NQ => String::from("queen"),
            CardNumber::NK => String::from("king"),
            CardNumber::NA => String::from("ace"),
            CardNumber::Empty => String::from("red_joker.png")
        };
        String::from("assets/") + &string1 + &string2
    }
}


pub enum Names {
    Player1, Player2, Player3, Player4, 
    Player5, Player6, Player7, Player8
}

pub enum CardState {
    Opened, Closed
}

pub enum PlayerPosition {
    Dealer, SmallBlind, BigBlind, UnderTheGun, 
    UnderTheGun1, MiddlePosition, Hijack, Cutoff,
    NotPlaying
}

pub struct Player {
    pub name: Names,
    pub card: Card,
    pub card_position: (i32, i32),
    pub card_state: CardState,
    pub position: PlayerPosition
    // later to be finished
}

impl Names {
    pub fn all_names() -> Vec<Names> {
        vec![Names::Player1, Names::Player2, Names::Player3, Names::Player4, 
        Names::Player5, Names::Player6, Names::Player7, Names::Player8]
    } 
}

impl Player {
    const PLAYER1_CARDS: (i32, i32) = (-775, 0);
    const PLAYER2_CARDS: (i32, i32) = (-500, 275);
    const PLAYER3_CARDS: (i32, i32) = (700, 0);
    const PLAYER4_CARDS: (i32, i32) = (500, 275);
    const PLAYER5_CARDS: (i32, i32) = (-50, 275);
    const PLAYER6_CARDS: (i32, i32) = (-50, -300);
    const PLAYER7_CARDS: (i32, i32) = (-500, -300);
    const PLAYER8_CARDS: (i32, i32) = (500, -300);

    pub fn init_players() -> Vec<Player> {
        let mut list_of_players = Vec::new();
        for name in Names::all_names() {
            let curr_player = Player {
                card_position: Self::get_card_position(&name),
                name,
                card: Card {
                    color: CardColor::Empty,
                    number: CardNumber::Empty
                },
                card_state: CardState::Opened,
                position: PlayerPosition::NotPlaying
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
}