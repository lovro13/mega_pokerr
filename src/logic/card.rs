use rand::seq::SliceRandom;
use rand::thread_rng;

#[derive(Clone, Debug, PartialEq, Eq)] // rabim clone da lahko naredim več kart, z istimi številkami
pub enum CardNumber {
    // označeno N ker je večini številk-Number, to sem si izmislil
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
    // mogoče ta del prestaviti v terminal_app ali pa mogoče res spada pod logiko, nisem se še odločil
    // mogoče bi bilo bolje da je v terminal_app, isto velja za karto in ostale std::fmt::Display
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
            CardNumber::Empty => panic!("trying to print EMPTY card!!!"),
        }
    }
}

impl CardNumber {
    pub fn evaluate_to_int(&self) -> u32 {
        match self {
            Self::N2 => 2,
            Self::N3 => 3,
            Self::N4 => 4,
            Self::N5 => 5,
            Self::N6 => 6,
            Self::N7 => 7,
            Self::N8 => 8,
            Self::N9 => 9,
            Self::N10 => 10,
            Self::NJ => 11,
            Self::NQ => 12,
            Self::NK => 13,
            Self::NA => 14,
            Self::Empty => {
                panic!("Empty card number, ko hočemo evaluirati CardNumber");
            }
        }
    }

    pub fn int_to_card_number(i: u32) -> CardNumber {
        match i {
            2 => CardNumber::N2,
            3 => CardNumber::N3,
            4 => CardNumber::N4,
            5 => CardNumber::N5,
            6 => CardNumber::N6,
            7 => CardNumber::N7,
            8 => CardNumber::N8,
            9 => CardNumber::N9,
            10 => CardNumber::N10,
            11 => CardNumber::NJ,
            12 => CardNumber::NQ,
            13 => CardNumber::NK,
            14 => CardNumber::NA,
            a => panic!("Invalid card number {}!", a),
        }
    }
}

impl PartialOrd for CardNumber {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.evaluate_to_int().cmp(&other.evaluate_to_int()))
    }
}

impl Ord for CardNumber {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.evaluate_to_int().cmp(&other.evaluate_to_int())
    }
}

#[derive(Clone, PartialEq)] // rabim clone da lahko naredim več kart, z istimi barvami
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
            CardColor::Empty => panic!("Empty card printed"),
        }
    }
}

impl std::fmt::Debug for CardColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self, f)
    }
}

#[derive(Debug, PartialEq, Clone)]
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
    pub fn new(number: &str, color: &str) -> Card {
        let number = match number {
            "2" => CardNumber::N2,
            "3" => CardNumber::N3,
            "4" => CardNumber::N4,
            "5" => CardNumber::N5,
            "6" => CardNumber::N6,
            "7" => CardNumber::N7,
            "8" => CardNumber::N8,
            "9" => CardNumber::N9,
            "10" => CardNumber::N10,
            "J" => CardNumber::NJ,
            "Q" => CardNumber::NQ,
            "K" => CardNumber::NK,
            "A" => CardNumber::NA,
            _ => CardNumber::Empty,
        };

        let color = match color {
            "H" => CardColor::Hearts,
            "S" => CardColor::Spades,
            "D" => CardColor::Diamonds,
            "C" => CardColor::Clubs,
            _ => CardColor::Empty,
        };

        Card { number, color }
    }
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

    pub fn sort_card_vec(vec: &mut Vec<Card>) {
        // na mestu sortira
        vec.sort_by(|a, b| a.number.cmp(&b.number));
    }

    pub fn next_in_straight(self) -> Card {
        if self.number == CardNumber::NA {
            Card {
                number: CardNumber::N2,
                color: self.color.clone(),
            }
        } else {
            println!("Card number: {}", CardNumber::int_to_card_number(self.number.evaluate_to_int()));
            let card_number = CardNumber::int_to_card_number(self.number.evaluate_to_int() + 1);
            Card {
                number: card_number,
                color: self.color.clone(),
            }
        }
    }
}
