use crate::logic::betting_system::make_bets;
use crate::logic::round::{begin_round, next_turn};
use crate::logic::game::init_game;
use crate::logic::player;
use crate::terminal_app::get_bet::get_bet;
use crate::terminal_app::prints::print_game_info;

fn main() {
    let players = player::Player::init_players();

    let game = init_game(players);
    let mut mut_game = game.borrow_mut();
    begin_round(&mut mut_game, 6);  
    print_game_info(&mut_game);
    make_bets(&mut mut_game, get_bet); // PREFLOP
    print_game_info(&mut_game);
    next_turn(&mut mut_game);
}