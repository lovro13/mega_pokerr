use crate::logic::card::CardNumber;

use super::card::Card;
use super::player::Player;
use super::round::Game;

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

pub fn choose_winner(game: &mut Game) -> &Player {
    println!("Hello");
    &game.players[0]
    // TODO TODO TODO
}

// treba rangirati karte
pub fn best_combination(
    _cards: (Card, Card),
    _table_cards: Vec<Card>,
) -> (RankOfHands, Vec<Card>) {
	// vrne naj po vrsti urejene karte ki niso bile uporabljene v kombinaciji,
	// oziroma prva ali prvi dve naj bosta te dve ki poveste kako visoka je bila kombinaicija

	(RankOfHands::RoyalFlush, vec![])
	// TODO TODO TODO
	// TODO TODO TODO
	// TODO TODO TODO
	// TODO TODO TODO
}

pub fn is_royal_flush(cards: &Vec<Card>) -> bool {
    assert!(
        cards.len() == 5,
        "Kart na mizi ni 5, ko hočemo določiti zmagovalca"
    );
    let color = &cards[0].color;
    for card in cards.iter() {
        if card.color != *color {
            return false;
        }
    }

    let mut values = Vec::new();
    for card in cards.iter() {	
		values.push(&card.number);
	}
	
	if values.contains(&&CardNumber::NA) == false {
		return false
	}
	if values.contains(&&CardNumber::NK) == false {
		return false
	}
	if values.contains(&&CardNumber::NQ) == false {
		return false
	}
	if values.contains(&&CardNumber::NJ) == false {
		return false
	}
	if values.contains(&&CardNumber::N10) == false {
		return false
	}

    true
}

pub fn is_straight_flush(cards: &Vec<Card>) -> bool {
	assert!(
		cards.len() == 5,
		"Kart na mizi ni 5, ko hočemo določiti zmagovalca"
	);
	let color = &cards[0].color;
	for card in cards.iter() {
		if card.color != *color {
			return false;
		}
	}

	let mut values = Vec::new();
	for card in cards.iter() {
		values.push(&card.number);
	}
	// TODO TODO TODO
	// TODO TODO TODO
	// TODO TODO TODO
	// TODO TODO TODO
	true
}

pub fn is_four_of_a_kind(cards: &Vec<Card>) -> bool {
	assert!(
		cards.len() == 5,
		"Kart na mizi ni 5, ko hočemo določiti zmagovalca"
	);
	let mut values = Vec::new();
	for card in cards.iter() {
		values.push(&card.number);
	}
	// TODO TODO TODO
	// TODO TODO TODO
	// TODO TODO TODO
	// TODO TODO TODO
	true
}

pub fn is_full_house(cards: &Vec<Card>) -> bool {
	assert!(
		cards.len() == 5,
		"Kart na mizi ni 5, ko hočemo določiti zmagovalca"
	);
	let mut values = Vec::new();
	for card in cards.iter() {
		values.push(&card.number);
	}
	// TODO TODO TODO
	// TODO TODO TODO
	// TODO TODO TODO
	// TODO TODO TODO
	true
}

pub fn is_flush(cards: &Vec<Card>) -> bool {
	assert!(
		cards.len() == 5,
		"Kart na mizi ni 5, ko hočemo določiti zmagovalca"
	);
	let color = &cards[0].color;
	for card in cards.iter() {
		if card.color != *color {
			return false;
		}
	}
	true
}

pub fn is_straight(cards: &Vec<Card>) -> bool {
	assert!(
		cards.len() == 5,
		);
		// TODO TODO TODO
		// TODO TODO TODO
		// TODO TODO TODO
		// TODO TODO TODO
	
	let mut values = Vec::new();
	for card in cards.iter() {
		values.push(&card.number);
	}
	// TODO TODO TODO
	// TODO TODO TODO
	// TODO TODO TODO
	// TODO TODO TODO
	true
}

pub fn is_three_of_a_kind(cards: &Vec<Card>) -> bool {
	assert!(
		cards.len() == 5,
		"Kart na mizi ni 5, ko hočemo določiti zmagovalca"
	);
	let mut values = Vec::new();
	for card in cards.iter() {
		values.push(&card.number);
	}
	// TODO TODO TODO
	// TODO TODO TODO
	// TODO TODO TODO
	// TODO TODO TODO
	true
}

pub fn is_two_pair(cards: &Vec<Card>) -> bool {
	assert!(
		cards.len() == 5,
		"Kart na mizi ni 5, ko hočemo določiti zmagovalca"
	);
	let mut values = Vec::new();
	for card in cards.iter() {
		values.push(&card.number);
	}
	// TODO TODO TODO
	// TODO TODO TODO
	// TODO TODO TODO
	// TODO TODO TODO
	true
}

pub fn is_one_pair(cards: &Vec<Card>) -> bool {
	assert!(
		cards.len() == 5,
		"Kart na mizi ni 5, ko hočemo določiti zmagovalca"
	);
	let mut values = Vec::new();
	for card in cards.iter() {
		values.push(&card.number);
	}
	// TODO TODO TODO
	// TODO TODO TODO
	// TODO TODO TODO
	// TODO TODO TODO
	true
}

