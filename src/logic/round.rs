use core::panic;

// treba bolj organizirati med round in game
use crate::logic::card;
use crate::logic::player;
use crate::logic::game::Game;
use crate::logic::game::Streets;

use super::constants::*;

pub struct Round {
    
}

pub fn begin_round(game: &mut Game, player_count: usize) {
    log::info!("Starting new round with {} players", player_count);
    // razdeli karte igralcem
    let deck = card::Card::make_ordered_deck();
    let mut deck = card::Card::scramble_deck(deck);
    game.pot = 0;
    for player in game.players.iter_mut() {
        if player.chips == 0 {
            player.chips = BUY_IN;
            player.debt += 1;
        }
        player.position = player::PlayerPosition::next_player_position_for_count(&player.position, player_count);
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
        player.hand_cards = (card1.clone(), card2.clone());
        log::debug!("Player {:?} at position {:?} got cards: {}, {}", player.id, player.position, card1, card2);
    }
    game.street = Streets::PreFlop;
    game.deck = deck;
    game.table_cards = Vec::new();
    
    // Nastavi začetno pozicijo glede na število igralcev
    game.position_on_turn = match player_count {
        2 => player::PlayerPosition::SmallBlind,
        3 => player::PlayerPosition::BigBlind,
        4..=8 => player::PlayerPosition::UnderTheGun,
        _ => player::PlayerPosition::UnderTheGun,
    };
    log::info!("Initial turn position set to {:?} for {} players", game.position_on_turn, player_count);
    game.round_number += 1;
}

pub fn next_turn(game: &mut Game) {
    log::debug!("Moving to next turn, current street: {:?}", game.street);
    // gre na naslednji street in "položi karte na mizo kolikor je treba"
    game.go_to_next_street();
    log::debug!("Street changed to: {:?}", game.street);
    
    let _ = match game.street.clone() {
        Streets::PreFlop => {
            log::debug!("PreFlop: no table cards to add");
        }
        Streets::Flop => {
            log::info!("Adding 3 cards to table (Flop)");
            for i in 0..3 {
                let card = match game.deck.pop() {
                    None => panic!("Deck is empty"),
                    Some(card) => card,
                };
                log::debug!("Adding card {} to table: {}", i + 1, card);
                game.table_cards.push(card);
            }
        }
        Streets::River | Streets::Turn => {
            let street_name = if game.street == Streets::Turn { "Turn" } else { "River" };
            log::info!("Adding 1 card to table ({})", street_name);
            let card = match game.deck.pop() {
                None => panic!("Deck is empty"),
                Some(card) => card,
            };
            log::debug!("Adding card to table: {}", card);
            game.table_cards.push(card);
        }
        Streets::Showdown => {
            log::info!("Showdown: no more cards to add");
        }
    };

    for player in game.players.iter_mut() {
        player.current_bet = 0;
        log::debug!("Reset current_bet for player {:?}", player.id);
    }
    log::debug!("Next turn completed, {} cards on table", game.table_cards.len());
}
