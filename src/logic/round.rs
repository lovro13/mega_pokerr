use core::panic;

// treba bolj organizirati med round in game
use crate::logic::card;
use crate::logic::choose_winner::choose_winner;
use crate::logic::player;
use crate::logic::game::Game;

#[derive(Debug, PartialEq, Clone)]
pub enum Streets {
    PreFlop,
    Flop,
    Turn,
    River,
    Showdown,
}

impl Streets {
    pub fn next(&self) -> Streets {
        match self {
            Streets::PreFlop => Streets::Flop,
            Streets::Flop => Streets::Turn,
            Streets::Turn => Streets::River,
            Streets::River => Streets::Showdown,
            Streets::Showdown => panic!("Game is over"),
        }
    }
}


pub fn begin_round(game: &mut Game) {
    // razdeli karte igralcem
    let deck = card::Card::make_ordered_deck();
    let mut deck = card::Card::scramble_deck(deck);
    for player in game.players.iter_mut() {
        player.position = player::PlayerPosition::next_player_position(&player.position);
        let card1 = match deck.pop() {
            None => panic!("Deck is empty (begin_round)"),
            Some(card) => card,
        };
        let card2 = match deck.pop() {
            None => panic!("Deck is empty (begin_round)"),
            Some(card) => card,
        };
        if player.position == player::PlayerPosition::SmallBlind {
            player.money -= 10;
            player.current_bet += 10;
        } else if player.position == player::PlayerPosition::BigBlind {
            player.money -= 20;
            player.current_bet += 20;
        }
        player.hand_cards = (card1, card2)
    }
    game.street = Streets::PreFlop;
    game.deck = deck;
    game.board_cards = Vec::new();
    game.position_on_turn = player::PlayerPosition::UnderTheGun;
    game.round_number += 1;
    game.pot = 30;
    game.players_in_game = vec![];
    for player in game.players.iter() {
        game.players_in_game.push(player.name.clone());
    }
}

pub fn next_turn(game: &mut Game) {
    // gre na naslednji street in "položi karte na mizo kolikor je treba"
    let _ = match game.street.clone() {
        Streets::PreFlop => {}
        Streets::Flop => {
            for _ in 0..2 {
                let card = match game.deck.pop() {
                    None => panic!("Deck is empty"),
                    Some(card) => card,
                };
                game.board_cards.push(card);
            }
        }
        Streets::River | Streets::Turn => {
            let card = match game.deck.pop() {
                None => panic!("Deck is empty"),
                Some(card) => card,
            };
            game.board_cards.push(card);
        }
        Streets::Showdown => {
            choose_winner(game);
        }
    };
    game.go_to_next_street();
}
