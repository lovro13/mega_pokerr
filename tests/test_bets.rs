
#[cfg(test)]
mod tests {
    // cargo test --test test_bets --features run_with_sdl2
    use mega_pokerr::logic::betting_system::make_bets;
    use mega_pokerr::logic::player::Player;
    use mega_pokerr::logic::game::init_game;
use rand::Rng;

    #[test]
    fn test_of_betting_function_runs() {
        let player_list = Player::init_players();
        let mut game = init_game(player_list);
        let get_bet = |player: &Player, bet: u32| {
            if player.playing {
                Some(bet)
            } else {
                None
            }
        };
        make_bets(&mut game, get_bet);
    }
    #[test]
    fn test_betting_all_in() {
        let player_list = Player::init_players();
        let mut game = init_game(player_list);
        let get_bet = |player: &Player, _bet: u32| {
            println!("Player {:?}, betting all in {}", player.position, player.chips);
            if player.playing {
                Some(player.chips) // Bet all chips
            } else {
                None
            }
        };
        println!("Game pot before: {}", game.pot);
        make_bets(&mut game, get_bet);
        println!("Game pot after: {}", game.pot);
    }

    #[test]
    fn test_betting_half_chips() {
        let player_list = Player::init_players();
        let mut game = init_game(player_list);
        let get_bet = |player: &Player, _bet: u32| {
            if player.playing {
                println!("Player {:?}, betting {}", player.position, player.chips / 2);
                Some(player.chips / 2) // Bet half of the chips
            } else {
                None
            }
        };
        println!("Game pot before: {}", game.pot);
        make_bets(&mut game, get_bet);
        println!("Game pot after: {}", game.pot);
    }

    #[test]
    fn test_betting_minimum() {
        let player_list = Player::init_players();
        let mut game = init_game(player_list);
        let get_bet = |player: &Player, _bet: u32| {
            if player.playing {
                Some(1) // Bet the minimum amount
            } else {
                None
            }
        };
        make_bets(&mut game, get_bet);
    }

    #[test]
    fn test_betting_random_amount() {
        let player_list = Player::init_players();
        let mut game = init_game(player_list);
        let get_bet = |player: &Player, _bet: u32| {
            if player.playing {
                let mut rng = rand::thread_rng();
                Some(rng.gen_range(1..=player.chips)) // Bet a random amount
            } else {
                None
            }
        };
        make_bets(&mut game, get_bet);
    }

    #[test]
    fn test_betting_zero_for_non_playing() {
        let player_list = Player::init_players();
        let mut game = init_game(player_list);
        let get_bet = |player: &Player, _bet: u32| {
            if player.playing {
                Some(10) // Bet a fixed amount
            } else {
                Some(0) // Non-playing players bet zero
            }
        };
        make_bets(&mut game, get_bet);
    }
}
