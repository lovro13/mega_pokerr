use crate::logic::{
    player::Player,
    card::{
        CardColor,
        CardNumber,
        Card
    },
};
use crate::logic::constants::{BIG_BLIND, SHOULD_QUIT, SHOULD_RETURN_TO_START};
use crate::logic::game::{
    Game,
    Streets
};
use crate::logic::tactics1::make_decision;



pub fn get_bet(game: &Game, req_bet: u32) -> Option<u32> {
    get_bet_1(game, req_bet, make_decision)
}


fn get_bet_1(game: &Game, req_bet: u32, 
    mut ai: impl FnMut(&(Card, Card), &Vec<Card>, u32, u32, u32) -> Option<u32>) -> Option<u32> {
        
        let table_cards = &game.table_cards;
        let player = game.player_on_turn_immutable();
        let players_curr_bet = player.current_bet;
        let player_hand = &player.hand_cards;
        let player_chips = player.chips;
        ai(player_hand, table_cards, req_bet, players_curr_bet, player_chips)
    }

fn ai_1(
    player_cards: &(Card, Card),
    table_cards: &Vec<Card>, 
    req_bet: u32,
    players_curr_bet: u32,
    player_chips: u32,
    ) -> Option<u32> {
        if players_curr_bet < 500 {Some(0)}
        else {None}
}