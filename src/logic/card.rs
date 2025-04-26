use rand::seq::SliceRandom;
use rand::thread_rng;

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

impl std::fmt::Display for CardNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CardNumber::N2 => write!(f, "2"),
            CardNumber::N3 => write!(f, "3"),
            CardNumber::N4 => write!(f, "4"),
            CardNumber::N5 => write!(f, "5"),
            CardNumber::N6 => write!(f, "6"),
            CardNumber::N7 => write!(f, "7"),
            CardNumber::N8 => write!(f, "8"),
            CardNumber::N9 => write!(f, "9"),
            CardNumber::N10 => write!(f, "10"),
            CardNumber::NJ => write!(f, "J"),
            CardNumber::NQ => write!(f, "Q"),
            CardNumber::NK => write!(f, "K"),
            CardNumber::NA => write!(f, "A"),
            CardNumber::Empty => write!(f, "EMPTY!!!"),
        }
    }
}

#[derive(Clone)] // rabim clone da lahko naredim več kart, z istimi barvami
pub enum CardColor {
    Hearts,
    Spades,
    Diamonds,
    Clubs,
    Empty,
}

impl std::fmt::Display for CardColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CardColor::Hearts => write!(f, "♥"),
            CardColor::Spades => write!(f, "♠"),
            CardColor::Diamonds => write!(f, "♦"),
            CardColor::Clubs => write!(f, "♣"),
            CardColor::Empty => write!(f, "EMPTY!!!"),
        }
    }
}

pub struct Card {
    pub color: CardColor,
    pub number: CardNumber,
}

impl std::fmt::Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.number, self.color)
    }
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


    // mogoče struct deck, in impl deck, da lahko definiramo metodo na decku
    pub fn scramble_deck(deck: Vec<Card>) -> Vec<Card> {
        let mut rng = thread_rng();
        let mut shuffled_deck = deck;
        shuffled_deck.shuffle(&mut rng);
        shuffled_deck
    }
}
