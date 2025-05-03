use crate::logic::card;
use crate::logic::player;
use crate::logic::player::Player;

use super::round::Streets;

pub struct Game {
    pub street: Streets,              // v bistvu pove koliko kart je na mizi
    pub pot: u32,                     // koliko je stav na mizi
    pub players: Vec<Player>, // seznam igralcev
    pub deck: Vec<card::Card>,        // seznam kart
    pub board_cards: Vec<card::Card>, // katere karte so na mizi
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
        board_cards: Vec::new(),
        position_on_turn: player::PlayerPosition::UnderTheGun,
        round_number: 0,
        players_in_game: players_in_game,
    }
}