use projektna_prog_2::logic::round::{begin_round, init_game, make_bets};
// use this app for debuging logic behind the game
use projektna_prog_2::terminal_app::prints::print_round;
use projektna_prog_2::terminal_app::get_bet::get_bet;
use projektna_prog_2::logic::player;

fn main() {
    let players = player::Player::init_players();

    let mut game = init_game(players);
    begin_round(&mut game);
    print_round(&game);
    make_bets(&mut game, get_bet);
    
    print_round(&game);
}

// po pavzi folding sistem, avtomatizirati se pomoje še ne splača