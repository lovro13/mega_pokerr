
#[cfg(test)]
mod tests {
    // cargo test --test test_bets --features run_with_sdl2
    use mega_pokerr::logic::betting_system::make_bets;
    use mega_pokerr::logic::player::Player;
    use mega_pokerr::logic::round::init_game;

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
