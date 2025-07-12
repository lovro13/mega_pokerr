use std::sync::atomic::Ordering;

use crate::logic::constants::BIG_BLIND;
use crate::logic::constants::SHOULD_QUIT;
use crate::logic::game::Game;
use crate::logic::game::Streets;
use crate::logic::player;

pub fn make_bets(game: &mut Game, mut get_bet: impl FnMut(&Game, u32) -> Option<u32>) {
    // pomoje bo to treba še enkrat napisati skor complete

    // ta funkcija naj bi v grobem na pravilen način zmanjšala player.money v game.players in povečala game.pot
    // če je treba kaj več vrniti, da bo koda v main
    // ali kje drugje bolj tekla ni nobenega problema

    // game.street ne bo spreminjala
    // plan:
    // - najprej dobi vse igralci ki so v igri, seznam game.players_in_game
    // - določi začetnega igralca
    // - določimo tak loop da se funkcija get_bet izvede za vsakega igralca, in to od začetnega <-
    // - in naslednji bo tisti, ki ima next player_position
    // - vsak igralec stavi enkrat, oziroma lahko folda, kar pomeni da ga izloči iz seznama igralcev, ki igrajo
    // - če je kdo stavil več kot ostali, je treba vse ostale igralce prisiliti, da stavijo še enkrat
    // - torej moramo vedeti koliko je največji bet

    // še eno zaprtje print info ki poskrbi kaj se izpiše na zaslonu
    log::info!("Starting make_bets");
    if game.street == Streets::PreFlop {
        game.position_on_turn = match game.player_count {
            2 => player::PlayerPosition::SmallBlind,
            3 => player::PlayerPosition::BigBlind,
            4..=8 => player::PlayerPosition::UnderTheGun,
            _ => player::PlayerPosition::UnderTheGun,
        };
        log::debug!("Set initial position for PreFlop: {:?} (player_count: {})", game.position_on_turn, game.player_count);
    } else {
        game.position_on_turn = player::PlayerPosition::SmallBlind;
        log::debug!("Set initial position for non-PreFlop: {:?}", game.position_on_turn);
    }

    let mut start_position = game.player_on_turn().position.clone();
    log::debug!("Starting betting round from position {:?}", start_position);

    let mut betting_players_pos = vec![start_position.clone()];
    game.go_to_next_player();

    while game.player_on_turn().position != start_position {
        if game.player_on_turn().playing {
            betting_players_pos.push(game.player_on_turn().position.clone())
        }
        game.go_to_next_player();
    }
    log::debug!("Found {} players in betting round", betting_players_pos.len());
    if betting_players_pos.len() <= 1 {
        log::warn!("Not enough players for betting round");
        return;
    }
    assert!(game.player_on_turn().position == start_position);

    let mut not_playing_players = vec![];

    let mut curr_highest_bet = 0;
    if game.street == Streets::PreFlop {
        curr_highest_bet = BIG_BLIND;
        log::debug!("PreFlop: starting with big blind amount {}", BIG_BLIND);
    }
    let mut need_another_round = false;
    loop {
        // loop če je treba narediti več krogov stav - torej ko nekdo raisa
        loop {
            // en krog stav, če nekdo raisa se krog konča in je on nov začetni player
            let player_pos = game.position_on_turn.clone();
            let curr_player = game.get_player_from_pos(&player_pos);
            log::debug!("Player {:?} at position {:?} is betting (chips: {}, current_bet: {})", 
                       curr_player.id, player_pos, curr_player.chips, curr_player.current_bet);
            
            if !curr_player.playing {
                log::debug!("Player {:?} is not playing, skipping", curr_player.id);
                game.go_to_next_player();
                if game.player_on_turn().position == start_position {
                    break;
                }
                continue;
            }
            let needed_bet = curr_highest_bet - curr_player.current_bet;
            log::debug!("Player {:?} needs to bet {}", curr_player.id, needed_bet);

            let bet = {
                let game_ref = &mut *game;
                get_bet(game_ref, needed_bet)
            };
            if SHOULD_QUIT.load(Ordering::Relaxed) {
                log::info!("Quit signal received during betting");
                return;
            }
            let curr_player = game.get_player_from_pos(&player_pos);
            match bet {
                None => {
                    // player folded
                    log::info!("Player {:?} folded", curr_player.id);
                    curr_player.current_bet = 0;
                    curr_player.playing = false;
                    not_playing_players.push(player_pos.clone());
                }
                Some(amount) if amount + curr_player.current_bet > curr_highest_bet => {
                    // player raised
                    log::info!("Player {:?} raised to {}", curr_player.id, amount + curr_player.current_bet);
                    curr_highest_bet = amount + curr_player.current_bet;
                    curr_player.chips -= amount;
                    curr_player.current_bet += amount;
                    game.pot += amount;
                    need_another_round = true;
                    start_position = player_pos.clone();
                    game.go_to_next_player();
                    break;
                }
                Some(amount) => {
                    // player called
                    log::debug!("Player {:?} called with {}", curr_player.id, amount);
                    curr_player.chips -= amount;
                    curr_player.current_bet += amount;
                    game.pot += amount;
                }
            }
            game.go_to_next_player();
            if game.player_on_turn().position == start_position {
                break;
            }
            let mut playing_players = 0;
            for player in game.players.iter() {
                if player.playing {
                    playing_players += 1;
                }
            }
            log::debug!("{} players still playing", playing_players);
            if playing_players <= 1 {
                log::info!("Only {} player(s) remaining, ending betting round", playing_players);
                return;
            }
        }

        if !need_another_round {
            log::debug!("No more betting rounds needed");
            break;
        }
        need_another_round = false;
        log::debug!("Starting another betting round due to raise");
    }
    log::info!("Betting round finished, pot: {}", game.pot);
}

pub fn validate_bet(req_bet: u32, chips: u32, bet: u32) -> bool {
    if req_bet > chips {
        return bet == chips;
    }
    req_bet <= bet && bet <= chips
} 