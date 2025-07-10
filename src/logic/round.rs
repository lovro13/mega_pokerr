use core::panic;

// treba bolj organizirati med round in game
use crate::logic::card;
use crate::logic::player;
use crate::logic::game::Game;
use crate::logic::game::Streets;

use super::constants::*;

pub struct Round {
    
}

pub fn begin_round(game: &mut Game) {
    // razdeli karte igralcem
    let deck = card::Card::make_ordered_deck();
    let mut deck = card::Card::scramble_deck(deck);
    game.pot = 0;
    for player in game.players.iter_mut() {
        if player.chips == 0 {
            player.chips = BUY_IN;
            player.debt += 1;
        }
        player.position = player::PlayerPosition::next_player_position(&player.position);
        player.playing = true;
        let card1 = match deck.pop() {
            None => panic!("Deck is empty (begin_round)"),
            Some(card) => card,
        };
        let card2 = match deck.pop() {
            None => panic!("Deck is empty (begin_round)"),
            Some(card) => card,
        };
        if player.position == player::PlayerPosition::SmallBlind {
            if player.chips < SMALL_BLIND {
                player.playing = false;
                game.pot += player.chips;
                player.chips = 0;
            } else {
                player.chips -= SMALL_BLIND;
                player.current_bet += SMALL_BLIND;
                game.pot += SMALL_BLIND;
            }
        } else if player.position == player::PlayerPosition::BigBlind {
            if player.chips < BIG_BLIND {
                player.playing = false;
                game.pot += player.chips;
                player.chips = 0;
            } else {
                player.chips -= BIG_BLIND;
                player.current_bet += BIG_BLIND;
                game.pot += BIG_BLIND;
            }
        }
        player.opened_cards = false;
        player.hand_cards = (card1, card2);
    }
    game.street = Streets::PreFlop;
    game.deck = deck;
    game.table_cards = Vec::new();
    game.position_on_turn = player::PlayerPosition::UnderTheGun;
    game.round_number += 1;
}

pub fn next_turn(game: &mut Game) {
    // gre na naslednji street in "položi karte na mizo kolikor je treba"
    game.go_to_next_street();
    let _ = match game.street.clone() {
        Streets::PreFlop => {}
        Streets::Flop => {
            for _ in 0..3 {
                let card = match game.deck.pop() {
                    None => panic!("Deck is empty"),
                    Some(card) => card,
                };
                game.table_cards.push(card);
            }
        }
        Streets::River | Streets::Turn => {
            let card = match game.deck.pop() {
                None => panic!("Deck is empty"),
                Some(card) => card,
            };
            game.table_cards.push(card);
        }
        Streets::Showdown => {
            
        }
    };

    for player in game.players.iter_mut() {
        player.current_bet = 0;
    }
}
