use crate::logic::constants::BIG_BLIND;
use crate::logic::constants::SMALL_BLIND;
use crate::logic::player;
use crate::logic::game::Game;
use crate::logic::game::Streets;
use crate::logic::player::PlayerPosition;

pub fn make_bets(game: &mut Game, get_bet: impl Fn(&player::Player, u32) -> Option<u32>) {
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

    for player in game.players.iter() {
        if player.position == PlayerPosition::BigBlind {
            assert!(player.current_bet == BIG_BLIND)
        } else if player.position == PlayerPosition::SmallBlind {
            assert!(player.current_bet == SMALL_BLIND)
        } else {
            assert!(player.current_bet == 0);
        }
    }
    
    if game.street == Streets::PreFlop {
        game.position_on_turn = player::PlayerPosition::UnderTheGun;
    } else {
        game.position_on_turn = player::PlayerPosition::SmallBlind;
    }

    let first_starting_player = game.player_on_turn().position.clone();
    let mut betting_players_pos = vec![first_starting_player.clone()];
    
    println!("\nDEBUG betting_system betting_players_pos: {:?}\n", betting_players_pos);
    
    let mut curr_highest_bet = BIG_BLIND;
    let mut need_another_round = false;
    let mut curr_start_player = first_starting_player.clone();
    loop {
        game.go_to_next_player();
        while game.player_on_turn().position != curr_start_player {
            if game.player_on_turn().playing {
                betting_players_pos.push(game.player_on_turn().position.clone())
            }
            game.go_to_next_player();
        }
        let mut not_playing_players = vec![];
        betting_players_pos.retain(|pos| !not_playing_players.contains(pos));
        assert!(curr_start_player == game.position_on_turn);
        for player_pos in betting_players_pos.iter() {
            let curr_player = game.get_player_from_pos(&player_pos);
            let needed_bet = curr_highest_bet - curr_player.current_bet;
            let bet = get_bet(curr_player, needed_bet);
            match bet {
                None => { // player folded
                    curr_player.current_bet = 0;
                    game.get_player_from_pos(&player_pos).playing = false;
                    not_playing_players.push(player_pos.clone());
                }
                Some(amount) if amount + curr_player.current_bet > curr_highest_bet => { // player raised
                    curr_highest_bet = amount + curr_player.current_bet;
                    curr_player.current_bet += amount;
                    game.pot += amount; 
                    need_another_round = true;
                    curr_start_player = player_pos.clone();
                    break;
                }
                Some(amount) => { // player called
                    curr_player.money -= amount;
                    curr_player.current_bet += amount;
                    game.pot += amount;
                }
            }
        }

        betting_players_pos.retain(|pos| !not_playing_players.contains(pos));
        if !need_another_round {
            break;
        }
        need_another_round = false;

    }
    // zdaj imam seznam igralcev ki igrajo
}
