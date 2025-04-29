use crate::logic::card::{Card, CardColor, CardNumber};

pub fn card_to_file(card: &Card) -> String {
    // sprejme karto in vrne ime fila
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
