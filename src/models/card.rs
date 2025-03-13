
pub enum CardNumber {
    // označeno R kot rang karte, to sem si izmislil
    // nevem če je to izraz
    N2, N3, N4, N5, N6, N7,
    N8, N9, N10, NJ, NQ, NK, NA    
}
pub enum CardColor {
    Hearts, Spades, Diamonds, Clubs 
}

pub struct Card {
    pub color: CardColor,
    pub number: CardNumber
}

impl Card {
    pub fn card_to_file(card: Card) -> String {
        let card_color = card.color;
        let card_num = card.number;
        let string2 = match card_color {
            CardColor::Hearts => String::from("_of_hearts.png"),
            CardColor::Spades => String::from("_of_spades.png"),
            CardColor::Diamonds => String::from("_of_diamonds.png"),
            CardColor::Clubs => String::from("_of_clubs.png")
        };
        let string1 = match card_num  {
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
            CardNumber::NA => String::from("ace")
        };
        String::from("assets/") + &string1 + &string2
    }
}