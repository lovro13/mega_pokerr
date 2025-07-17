use crate::logic::card::CardNumber;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HandRanking {
    RoyalFlush(Vec<CardNumber>),
    StraightFlush(Vec<CardNumber>),
    FourOfAKind(Vec<CardNumber>), // prva karta je karte ki so 4 iste, druga pa tista ki je ostala
    FullHouse(Vec<CardNumber>), // prva karta je karte ki so 3 iste, druga pa tista ki je ostala
    Flush(Vec<CardNumber>), // prva karta je tista ki je najvišja
    Straight(Vec<CardNumber>), // prva karta je tista ki je najvišja
    ThreeOfAKind(Vec<CardNumber>), // prva karta je tista ki je 3 iste, ostale pa so ostale
    TwoPair(Vec<CardNumber>), // prva karta je tista ki je 2 iste, druga pa tista ki je 2 iste, ostale pa so ostale
    OnePair(Vec<CardNumber>), // prva karta je tista ki je 2 iste, ostale pa so ostale
    HighCard(Vec<CardNumber>), // vseh 5 urjenejih po velikosti
}

impl HandRanking {
    pub fn rank_value(&self) -> i32 {
        match self {
            HandRanking::RoyalFlush(_) => 10,
            HandRanking::StraightFlush(_) => 9,
            HandRanking::FourOfAKind(_) => 8,
            HandRanking::FullHouse(_) => 7,
            HandRanking::Flush(_) => 6,
            HandRanking::Straight(_) => 5,
            HandRanking::ThreeOfAKind(_) => 4,
            HandRanking::TwoPair(_) => 3,
            HandRanking::OnePair(_) => 2,
            HandRanking::HighCard(_) => 1,
        }
    }
}

impl PartialOrd for HandRanking {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for HandRanking {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Primerjaj po moči kombinacije
        let self_rank = self.rank_value();
        let other_rank = other.rank_value();
        
        match self_rank.cmp(&other_rank) {
            // Če sta kombinaciji različni, vrni rezultat
            std::cmp::Ordering::Equal => {
                // Če sta kombinaciji enaki, primerjaj karte po vrsti
                match (self, other) {
                    (HandRanking::RoyalFlush(_), HandRanking::RoyalFlush(_)) => std::cmp::Ordering::Equal,
                    (HandRanking::StraightFlush(a), HandRanking::StraightFlush(b)) => compare_card_vectors(a, b),
                    (HandRanking::FourOfAKind(a), HandRanking::FourOfAKind(b)) => compare_card_vectors(a, b),
                    (HandRanking::FullHouse(a), HandRanking::FullHouse(b)) => compare_card_vectors(a, b),
                    (HandRanking::Flush(a), HandRanking::Flush(b)) => compare_card_vectors(a, b),
                    (HandRanking::Straight(a), HandRanking::Straight(b)) => compare_card_vectors(a, b),
                    (HandRanking::ThreeOfAKind(a), HandRanking::ThreeOfAKind(b)) => compare_card_vectors(a, b),
                    (HandRanking::TwoPair(a), HandRanking::TwoPair(b)) => compare_card_vectors(a, b),
                    (HandRanking::OnePair(a), HandRanking::OnePair(b)) => compare_card_vectors(a, b),
                    (HandRanking::HighCard(a), HandRanking::HighCard(b)) => compare_card_vectors(a, b),
                    _ => unreachable!(), // Zagotovljeno zaradi prejšnjega matcha na rank_value
                }
            }
            // Če sta kombinaciji različni, vrni rezultat
            ordering => ordering,
        }
    }
}

// Pomožna funkcija za primerjavo vektorjev kart
fn compare_card_vectors(a: &[CardNumber], b: &[CardNumber]) -> std::cmp::Ordering {
    a.iter()
        .zip(b.iter())
        .map(|(a_card, b_card)| a_card.cmp(b_card))
        .find(|&ordering| ordering != std::cmp::Ordering::Equal)
        .unwrap_or(std::cmp::Ordering::Equal)
}
