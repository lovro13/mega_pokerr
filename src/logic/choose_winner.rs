// File: src/logic/choose_winner.rs

use crate::logic::combinations::*;

use super::card::{Card, CardNumber};
use super::player::Player;
use super::round::Game;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RankOfHands {
    RoyalFlush(Vec<CardNumber>),
    StraightFlush(Vec<CardNumber>),
    FourOfAKind(Vec<CardNumber>),
    FullHouse(Vec<CardNumber>),
    Flush(Vec<CardNumber>),
    Straight(Vec<CardNumber>),
    ThreeOfAKind(Vec<CardNumber>),
    TwoPair(Vec<CardNumber>),
    OnePair(Vec<CardNumber>),
    HighCard(Vec<CardNumber>),
}


pub fn choose_winner(game: &mut Game) -> &Player {
    println!("Hello");
    &game.players[0]
    // TODO TODO TODO
}

// treba rangirati karte
pub fn best_combination(
	cards: &mut Vec<Card>,
) -> RankOfHands {
	// vrne naj po vrsti urejene karte ki niso bile uporabljene v kombinaciji,
	// oziroma prva ali prvi dve naj bosta te dve ki poveste kako visoka je bila kombinaicija
	// TODO: urediti nums da bodo primerni za primerjanje
	assert!(
		cards.len() == 5,
		"Na mizi ni 5 kart, ko hočemo določiti zmagovalca (best_combination)"
	);
	if is_royal_flush(cards) {
		let mut nums = Vec::new();
		for card in cards.iter() {
			nums.push(card.number.clone());
		}
		nums.sort();
		return RankOfHands::RoyalFlush(nums);
	} else if is_straight_flush(cards) {
		let mut nums = Vec::new();
		for card in cards.iter() {
			nums.push(card.number.clone());
		}
		nums.sort();
		return RankOfHands::StraightFlush(nums);
	} else if is_four_of_a_kind(cards) {
		let mut nums = Vec::new();
		for card in cards.iter() {
			nums.push(card.number.clone());
		}
		nums.sort();
		return RankOfHands::FourOfAKind(nums);
	} else if is_full_house(cards) {
		let mut nums = Vec::new();
		for card in cards.iter() {
			nums.push(card.number.clone());
		}
		nums.sort();
		return RankOfHands::FullHouse(nums);
	} else if is_flush(cards) {
		let mut nums = Vec::new();
		for card in cards.iter() {
			nums.push(card.number.clone());
		}
		nums.sort();
		return RankOfHands::Flush(nums);
	} else if is_straight(cards) {
		let mut nums = Vec::new();
		for card in cards.iter() {
			nums.push(card.number.clone());
		}
		nums.sort();
		return RankOfHands::Straight(nums);
	} else if is_three_of_a_kind(cards) {
		let mut nums = Vec::new();
		for card in cards.iter() {
			nums.push(card.number.clone());
		}
		nums.sort();
		return RankOfHands::ThreeOfAKind(nums);
	} else if is_two_pair(cards) {
		let mut nums = Vec::new();
		for card in cards.iter() {
			nums.push(card.number.clone());
		}
		nums.sort();
		return RankOfHands::TwoPair(nums);
	} else if is_one_pair(cards) {
		let mut nums = Vec::new();
		for card in cards.iter() {
			nums.push(card.number.clone());
		}
		nums.sort();
		return RankOfHands::OnePair(nums);
	}
	RankOfHands::RoyalFlush(vec![])
	// TODO TODO TODO
	// TODO TODO TODO
	// TODO TODO TODO
	// TODO TODO TODO
	// da bojo nums vrnjeni pravilno razvrščeni za "abecedno" primerijanje med sabo
}




