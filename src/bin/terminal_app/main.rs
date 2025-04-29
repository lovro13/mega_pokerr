use projektna_prog_2::logic::betting_system::make_bets;
use projektna_prog_2::logic::round::{begin_round, init_game};
// use this app for debuging logic behind the game
use projektna_prog_2::logic::player;
use projektna_prog_2::terminal_app::get_bet::get_bet;
use projektna_prog_2::terminal_app::prints::print_game_info;

fn main() {
    let players = player::Player::init_players();

    let mut game = init_game(players);
    begin_round(&mut game);
    print_game_info(&game);
    make_bets(&mut game, get_bet);

    print_game_info(&game);
}
