#[cfg(test)]

// cargo test --test test_bets --features run_with_sdl2
mod tests {
    use projektna_prog_2::logic::betting_system::make_bets;
    use projektna_prog_2::logic::player::Player;
    use projektna_prog_2::logic::round::init_game;

    #[test]
    fn test_of_betting_function_runs() {
        let player_list = Player::init_players();
        let mut game = init_game(player_list);
        let get_bet = |player: &Player| {
            if player.playing {
                Some(0)
            } else {
                None
            }
        };
        make_bets(&mut game, get_bet);
    }
}
