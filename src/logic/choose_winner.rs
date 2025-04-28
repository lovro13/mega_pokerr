// File: src/logic/choose_winner.rs
use crate::logic::combinations::RankOfHands;
use super::card::Card;
use super::player::Player;
use super::round::Game;


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



