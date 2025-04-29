use core::panic;

// treba bolj organizirati med round in game
use crate::logic::card;
use crate::logic::choose_winner::choose_winner;
use crate::logic::player;

use super::player::Player;

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
pub struct Game {
    pub street: Streets,              // v bistvu pove koliko kart je na mizi
    pub pot: u32,                     // koliko je stav na mizi
    pub players: Vec<player::Player>, // seznam igralcev
    pub deck: Vec<card::Card>,        // seznam kart
    pub table_cards: Vec<card::Card>, // katere karte so na mizi
    pub position_on_turn: player::PlayerPosition, // kateri igralec je na vrsti, imamo poziicijo, torej kje sedi
    pub round_number: u32,                      // okrasek, koliko rund smo že odigrali
    pub players_in_game: Vec<player::Names>,    // koliko igralcev še ni foldalo
}

impl Game {
    pub fn go_to_next_street(&mut self) {
        self.street = self.street.next()
    }

    pub fn go_to_next_player(&mut self) {
        self.position_on_turn = self.position_on_turn.next_player_position();
    }

    pub fn player_on_turn(&mut self) -> &mut Player {
        for player in self.players.iter_mut() {
            if player.position == self.position_on_turn {
                return player;
            }
        }
        panic!("Player not found (go_to_next_street)");
    }

    pub fn get_player_from_pos(&mut self, pos: &player::PlayerPosition) -> &mut Player {
        for player in self.players.iter_mut() {
            if player.position == *pos {
                return player
            }
        }
        panic!("Pozicija manjka (get_player_from_pos)")
    }
}

pub fn init_game(player_list: Vec<player::Player>) -> Game {
    let deck = card::Card::make_ordered_deck();
    let deck = card::Card::scramble_deck(deck);
    let mut players_in_game = vec![];
    for player in player_list.iter() {
        players_in_game.push(player.name.clone());
    }
    let mut_player_list = player_list;
    Game {
        street: Streets::PreFlop,
        pot: 30,
        players: mut_player_list,
        deck,
        table_cards: Vec::new(),
        position_on_turn: player::PlayerPosition::UnderTheGun,
        round_number: 0,
        players_in_game: players_in_game,
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
        } else if player.position == player::PlayerPosition::BigBlind {
            player.money -= 20;
        }
        player.cards = (card1, card2)
    }
    game.street = Streets::PreFlop;
    game.deck = deck;
    game.table_cards = Vec::new();
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
            choose_winner(game);
        }
    };
    game.go_to_next_street();
}
