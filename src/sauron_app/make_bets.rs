
use crate::logic::game::{
    Game,
};
use crate::logic::player::{PlayerPosition, Id};
use crate::sauron_app::constants::MAIN_PLAYER;

#[derive(Clone, Debug)]
pub enum Response {
    WaitingPlayer1(u32),
    OnePlayerRemaning,
    StreetFinished(Id, Option<u32>),
    BetPlaced(Id, u32, Option<u32>),
    EndRound,
}

pub fn fold_bet(game: &mut Game) -> Response {
    let player_id = {
        let player = game.player_on_turn();
        player.playing = false;
        player.id.clone()
    };
    game.position_on_turn = game.position_on_turn.next_player_on_turn_for_count(game.player_count);
    let finished = if game.round_number as usize > game.player_count {true} else {false};
    
    if finished {
        game.position_on_turn = PlayerPosition::SmallBlind;
        Response::StreetFinished(player_id, None)
    } else {
        Response::BetPlaced(player_id, 0, None)
    }
}

pub fn active_bet(game: &mut Game, current_req_bet: u32, raise: u32) -> Response {
    let player_id = {
        let player = game.player_on_turn_immutable();
        player.id.clone()
    };
    if raise != 0 {game.round_number = 2};
    let new_req_bet = current_req_bet + raise;
    let (actual_raise, actual_diff)= {
        let player = game.player_on_turn();
        let diff = new_req_bet - player.current_bet;
        if player.chips < diff {
            // All-in
            let remaining = player.chips;
            let all_in_amount = player.current_bet + player.chips;
            player.current_bet = all_in_amount;
            player.chips = 0;
            (remaining - current_req_bet, remaining)
        } else {
            // Normal bet
            player.current_bet = new_req_bet;
            player.chips -= diff;
            (raise, diff)
        }
    };
    game.pot += actual_diff;
    game.position_on_turn = game.position_on_turn.next_player_on_turn_for_count(game.player_count);
    
    // Check if round is finished
    let finished = if game.round_number as usize > game.player_count {true} else {false};
    let max_bet = game.players.iter()
        .filter(|p| p.playing)
        .map(|p| p.current_bet)
        .max()
        .unwrap_or(0);
    
    
    if finished {
        game.position_on_turn = PlayerPosition::SmallBlind;
        Response::StreetFinished(player_id, Some(actual_raise))
    } else {
        Response::BetPlaced(player_id, max_bet, Some(actual_raise))
    }
}

pub fn make_bets(game: &mut Game, req_bet: u32, mut get_bet: impl FnMut(&Game, u32) -> Option<u32>) -> Response {
    game.round_number += 1;
    let (player_id, is_playing) = {
        let player = game.player_on_turn_immutable();
        (player.id.clone(), player.playing)
    };
    let active_players_count = game.players.iter().filter(|p| p.playing).count();
    if active_players_count <= 1 {
        return Response::OnePlayerRemaning;
    };
    if is_playing {
        if player_id == MAIN_PLAYER {
            Response::WaitingPlayer1(req_bet)
        } else {
            match get_bet(&game, req_bet) {
                None => {
                    fold_bet(game)
                },
                Some(raise) => {
                    active_bet(game, req_bet, raise)
                }
            }
        }
    } else {
        game.position_on_turn = game.position_on_turn.next_player_on_turn_for_count(game.player_count);
        let finished = if game.round_number as usize > game.player_count {true} else {false};
        if finished {
            Response::EndRound
        } else {
            make_bets(game, req_bet, get_bet)
        }
        
    }


    
    
}