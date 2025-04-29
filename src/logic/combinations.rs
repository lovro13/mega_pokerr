use crate::logic::card::{Card, CardNumber};

pub enum RankOfHands {
    RoyalFlush,
    StraightFlush,
    FourOfAKind,
    FullHouse,
    Flush,
    Straight,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

pub fn is_one_pair(cards: &Vec<Card>) -> bool {
    assert!(
        cards.len() == 5,
        "Kart na mizi ni 5, ko hočemo določiti zmagovalca (is_one_pair)"
    );
    let mut values = Vec::new();
    for card in cards.iter() {
        values.push(&card.number);
    }

    for value in values.iter() {
        let matching_numbers = values.iter().filter(|&v| v == value);
        if matching_numbers.count() == 2 {
            return true;
        }
    }
    false
}

pub fn is_straight(cards: &mut Vec<Card>) -> bool {
    assert!(
        cards.len() == 5,
        "Kart na mizi ni 5, ko hočemo določiti zmagovalca (is_straight)"
    );
    Card::sort_card_vec(cards);
    if cards[0].number == CardNumber::N2
        && cards[1].number == CardNumber::N3
        && cards[2].number == CardNumber::N4
        && cards[3].number == CardNumber::N5
        && cards[4].number == CardNumber::NA
    {
        return true;
    } // robni primer je samo en pomoje tak ki narobe sortira za straight
    for i in 1..5 {
        if cards[i].number.evaluate_to_int() != cards[i - 1].number.evaluate_to_int() + 1 {
            return false;
        }
    }

    true
}

pub fn is_royal_flush(cards: &mut Vec<Card>) -> bool {
    assert!(
        cards.len() == 5,
        "Kart na mizi ni 5, ko hočemo določiti zmagovalca (is_royal_flush)"
    );
    let color = &cards[0].color;
    for card in cards.iter() {
        if card.color != *color {
            return false;
        }
    }

    is_straight(cards)
        && cards[0].number == CardNumber::N10
        && cards[1].number == CardNumber::NJ
        && cards[2].number == CardNumber::NQ
        && cards[3].number == CardNumber::NK
        && cards[4].number == CardNumber::NA
}

pub fn is_straight_flush(cards: &mut Vec<Card>) -> bool {
    assert!(
        cards.len() == 5,
        "Kart na mizi ni 5, ko hočemo določiti zmagovalca (is_straight_flush)"
    );
    let color = &cards[0].color;
    for card in cards.iter() {
        if card.color != *color {
            return false;
        }
    }

    is_straight(cards)
}

pub fn is_four_of_a_kind(cards: &Vec<Card>) -> bool {
    assert!(
        cards.len() == 5,
        "Kart na mizi ni 5, ko hočemo določiti zmagovalca (is_four_of_a_kind)"
    );
    let mut values = Vec::new();
    for card in cards.iter() {
        values.push(&card.number);
    }
    for value in values.iter() {
        if values.iter().filter(|&v| v == value).count() == 4 {
            return true;
        }
    }
    false
}

pub fn is_full_house(cards: &Vec<Card>) -> bool {
    assert!(
        cards.len() == 5,
        "Kart na mizi ni 5, ko hočemo določiti zmagovalca (is_full_house)"
    );
    let mut values = Vec::new();
    for card in cards.iter() {
        values.push(&card.number);
    }
    for value in values.iter() {
        let matching_numbers: Vec<_> = values.iter().filter(|&v| v == value).collect();
        if matching_numbers.len() == 3 {
            let mut remaining_values = values.clone();
            remaining_values.retain(|&v| *v != **value);
            if remaining_values.len() == 2 {
                let second_value = &remaining_values[0];
                if remaining_values
                    .iter()
                    .filter(|&&v| *v == **second_value)
                    .count()
                    == 2
                {
                    return true;
                }
            }
        }
    }
    false
}

pub fn is_flush(cards: &Vec<Card>) -> bool {
    assert!(
        cards.len() == 5,
        "Kart na mizi ni 5, ko hočemo določiti zmagovalca (is_flush)"
    );
    let color = &cards[0].color;
    for card in cards.iter() {
        if card.color != *color {
            return false;
        }
    }
    true
}

pub fn is_three_of_a_kind(cards: &Vec<Card>) -> bool {
    assert!(
        cards.len() == 5,
        "Kart na mizi ni 5, ko hočemo določiti zmagovalca (is_three_of_a_kind)"
    );
    let mut values = Vec::new();
    for card in cards.iter() {
        values.push(&card.number);
    }
    
    for value in values.iter() {
        let matching_numbers = values.iter().filter(|&v| v == value);
        if matching_numbers.count() == 3 {
            return true;
        }
    }
    false
}

pub fn is_two_pair(cards: &Vec<Card>) -> bool {
    assert!(
        cards.len() == 5,
        "Kart na mizi ni 5, ko hočemo določiti zmagovalca (is_two_pair)"
    );
    let mut values = Vec::new();
    for card in cards.iter() {
        values.push(&card.number);
    }
    for value in values.iter() {
        let matching_numbers = values.iter().filter(|&v| **v == **value);
        if matching_numbers.count() == 2 {
            let mut remaining_values = values.clone();
            remaining_values.retain(|&v| *v != **value);
            if remaining_values.len() == 3 {
                for second_value in remaining_values.iter() {
                    let second_matching_numbers = remaining_values
                        .iter()
                        .filter(|&&v| *v == **second_value);
                    if second_matching_numbers.count() == 2 {
                        return true;
                    }
                }
            }
        }
    }
    false
}
