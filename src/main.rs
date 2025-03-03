enum CardNumber {
    // označeno R kot rang karte, to sem si izmislil
    // nevem če je to izraz
    N2, N3, N4, N5, N6, N7,
    N8, N9, N10, NJ, RQ, NK, NA    
}
enum CardColor {
    Hearts, Spades, Diamonds, Clubs 
}

struct Card {
    Color: CardColor,
    Number: CardNumber
}


fn main() {
    println!("Hello, world!");
}
