// use this terminal app for debuging logic behind the game
use mega_pokerr::logic::betting_system::make_bets;
use mega_pokerr::logic::round::{begin_round, next_turn};
use mega_pokerr::logic::game::init_game;
use mega_pokerr::logic::player;
use mega_pokerr::terminal_app::get_bet::get_bet;
use mega_pokerr::terminal_app::prints::print_game_info;


fn main() {
    let players = player::Player::init_players();

    let game = init_game(players);
    let mut mut_game = game.borrow_mut();
    begin_round(&mut mut_game,6); // Magic number 6 = # of players
    print_game_info(&mut_game);
    make_bets(&mut mut_game, get_bet); // PREFLOP
    print_game_info(&mut_game);
    next_turn(&mut mut_game);
}
