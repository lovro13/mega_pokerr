// use this terminal app for debuging logic behind the game
use mega_pokerr::logic::betting_system::make_bets;
use mega_pokerr::logic::round::{begin_round, init_game};
use mega_pokerr::logic::player;
use mega_pokerr::terminal_app::get_bet::get_bet;
use mega_pokerr::terminal_app::prints::print_game_info;

fn main() {
    let players = player::Player::init_players();

    let mut game = init_game(players);
    begin_round(&mut game);
    print_game_info(&game);
    make_bets(&mut game, get_bet);

    print_game_info(&game);
}
