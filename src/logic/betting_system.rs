use crate::logic::constants::BIG_BLIND;
use crate::logic::game::Game;
use crate::logic::game::Streets;
use crate::logic::player;


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


    if game.street == Streets::PreFlop {
        game.position_on_turn = player::PlayerPosition::UnderTheGun;
    } else {
        game.position_on_turn = player::PlayerPosition::SmallBlind;
    }

    let mut start_position = game.player_on_turn().position.clone();

    let mut betting_players_pos = vec![start_position.clone()];
    game.go_to_next_player();

    while game.player_on_turn().position != start_position {
        if game.player_on_turn().playing {
            betting_players_pos.push(game.player_on_turn().position.clone())
        }
        game.go_to_next_player();
    }

    println!(
        "\nDEBUG betting_system betting_players_pos: {:?}\n",
        betting_players_pos
    );
    assert!(game.player_on_turn().position == start_position);

    let mut not_playing_players = vec![];

    let mut curr_highest_bet = 0;
    if game.street == Streets::PreFlop {
        curr_highest_bet = BIG_BLIND;
    }
    let mut need_another_round = false;
    loop {
        // loop če je treba narediti več krogov stav - torej ko nekdo raisa
        loop {
            // en krog stav, če nekdo raisa se krog konča in je on nov začetni player
            if not_playing_players.len() >= 7 {
                return;
            }
            println!("\n=====DEBUG=======================");
            println!(
                "curr player : {:?}  \nstart pos : {:?}",
                game.player_on_turn().position,
                start_position
            );
            let player_pos = game.position_on_turn.clone();
            let curr_player = game.get_player_from_pos(&player_pos);
            if !curr_player.playing {
                game.go_to_next_player();
                if game.player_on_turn().position == start_position {
                    break;
                }
                continue;
            }
            let needed_bet = curr_highest_bet - curr_player.current_bet;
            let bet = get_bet(curr_player, needed_bet);
            match bet {
                None => {
                    // player folded
                    curr_player.current_bet = 0;
                    curr_player.playing = false;
                    not_playing_players.push(player_pos.clone());
                }
                Some(amount) if amount + curr_player.current_bet > curr_highest_bet => {
                    // player raised
                    curr_highest_bet = amount + curr_player.current_bet;
                    curr_player.current_bet += amount;
                    game.pot += amount;
                    need_another_round = true;
                    start_position = player_pos.clone();
                    game.go_to_next_player();
                    break;
                }
                Some(amount) => {
                    // player called
                    curr_player.chips -= amount;
                    curr_player.current_bet += amount;
                    game.pot += amount;
                }
            }
            game.go_to_next_player();
            if game.player_on_turn().position == start_position {
                break;
            }
        }

        if !need_another_round {
            break;
        }
        need_another_round = false;
    }
    // zdaj imam seznam igralcev ki igrajo
}
