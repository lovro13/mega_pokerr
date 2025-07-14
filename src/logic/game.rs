use core::panic;

use crate::logic::card;
use crate::logic::player;
use crate::logic::player::Player;
use std::rc::Rc;
use std::cell::RefCell;
use serde::{Deserialize, Serialize};


use super::player::Id;


#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
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


#[derive(Serialize, Deserialize, Debug)]
pub struct Game {
    pub street: Streets,              // v bistvu pove koliko kart je na mizi
    pub pot: u32,                     // koliko je stav na mizi
    pub players: Vec<Player>,         // seznam igralcev
    pub deck: Vec<card::Card>,        // seznam kart
    pub table_cards: Vec<card::Card>, // katere karte so na mizi
    pub position_on_turn: player::PlayerPosition, // kateri igralec je na vrsti, imamo poziicijo, torej kje sedi
    pub round_number: u32,
    pub quit: bool,                   // okrasek, koliko rund smo že odigral
    pub player_count: usize,          // število igralcev v igri
}

impl Game {
    pub fn go_to_next_street(&mut self) {
        self.street = self.street.next()
    }

    pub fn go_to_next_player(&mut self) {
        // uporablja se v make_bets v while true loopu namesto for zanka
        // ker je vsakič drugi začetni igralec
    
        let next_player = self.position_on_turn.next_player_on_turn_for_count(self.player_count);
        log::debug!("Moving from position {:?} to {:?} (player_count: {})", self.position_on_turn, next_player, self.player_count);
        self.position_on_turn = next_player;
    }

    pub fn player_on_turn(&mut self) -> &mut Player {
        // isto se uporablja samo v make_bets
        for player in self.players.iter_mut() {
            if player.position == self.position_on_turn {
                return player;
            }
        }
        panic!("Player not found (go_to_next_street)");
    }

    pub fn player_on_turn_immutable(&self) -> &Player {
        // isto se uporablja samo v make_bets
        for player in self.players.iter() {
            if player.position == self.position_on_turn {
                return player;
            }
        }
        panic!("Player not found (go_to_next_street)");
    }

    pub fn get_player_from_pos(&mut self, pos: &player::PlayerPosition) -> &mut Player {
        // isto se uporablja samo v make_bets
        for player in self.players.iter_mut() {
            if player.position == *pos {
                return player;
            }
        }
        panic!("Pozicija manjka (get_player_from_pos)")
    }
    pub fn get_player_from_name(&mut self, name: Id) -> &mut Player {
        for player in self.players.iter_mut() {
            if player.id == name {
                return player;
            }
        }
        panic!("Player with name '{:?}' not found (get_player_from_name)", name);
    }
}

pub fn init_game(player_list: Vec<player::Player>) -> Rc<RefCell<Game>> {
    let deck = card::Card::make_ordered_deck();
    let deck = card::Card::scramble_deck(deck);
    let player_count = player_list.len();
    let mut players_in_game = vec![];
    for player in player_list.iter() {
        players_in_game.push(player.id.clone());
    }
    let mut_player_list = player_list;
    Rc::new(RefCell::new(Game {
        street: Streets::PreFlop,
        pot: 0,
        players: mut_player_list,
        deck,
        table_cards: Vec::new(),
        position_on_turn: player::PlayerPosition::UnderTheGun,
        round_number: 0,
        quit: false,
        player_count
    }))
}
