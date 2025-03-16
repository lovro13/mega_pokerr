use rand::seq::SliceRandom;
use rand::thread_rng;


pub const CARD_HEIGHT: u32 = 120;
pub const CARD_WIDTH: u32 = 95;

#[derive(Clone)] // rabim clone da lahko naredim več kart, z istimi številkami
pub enum CardNumber {
    // označeno R kot rang karte, to sem si izmislil
    // nevem če je to izraz
    N2,
    N3,
    N4,
    N5,
    N6,
    N7,
    N8,
    N9,
    N10,
    NJ,
    NQ,
    NK,
    NA,
    Empty,
}

#[derive(Clone)] // rabim clone da lahko naredim več kart, z istimi barvami
pub enum CardColor {
    Hearts,
    Spades,
    Diamonds,
    Clubs,
    Empty,
}

pub struct Card {
    pub color: CardColor,
    pub number: CardNumber,
}

impl Card {
    pub fn all_colors() -> Vec<CardColor> {
        vec![
            CardColor::Hearts,
            CardColor::Spades,
            CardColor::Diamonds,
            CardColor::Clubs,
        ]
    }

    pub fn all_numbers() -> Vec<CardNumber> {
        vec![
            CardNumber::N2,
            CardNumber::N3,
            CardNumber::N4,
            CardNumber::N5,
            CardNumber::N6,
            CardNumber::N7,
            CardNumber::N8,
            CardNumber::N9,
            CardNumber::N10,
            CardNumber::NJ,
            CardNumber::NQ,
            CardNumber::NK,
            CardNumber::NA,
        ]
    }

    pub fn make_ordered_deck() -> Vec<Card> {
        let mut all = Vec::new();
        for number in Self::all_numbers() {
            for color in Self::all_colors() {
                all.push(Card {
                    number: number.clone(),
                    color: color.clone(),
                })
            }
        }
        all
    }

    pub fn scramble_deck(deck: Vec<Card>) -> Vec<Card> {
        let mut rng = thread_rng();
        let mut shuffled_deck = deck;
        shuffled_deck.shuffle(&mut rng);
        shuffled_deck
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
            CardColor::Empty => String::from(""),
        };
        let string1 = match number {
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
            CardNumber::Empty => String::from("red_joker.png"),
        };
        String::from("assets/") + &string1 + &string2
    }
}

pub enum CardState {
    Opened,
    Closed,
}
