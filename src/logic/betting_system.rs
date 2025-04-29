use crate::logic::player;
use crate::logic::round::Game;

pub fn make_bets(game: &mut Game, get_bet: impl Fn(&player::Player) -> Option<u32>) {
    // ta funkcija naj bi v grobem na pravilen način zmanjšala player.money v game.players in povečala game.pot
    // če je treba še kaj več vrniti ni problema

    // game.street ne bo spreminjala
    // current postion bo zelo spreminjača

    // TODO: narediti loop, da se bo stavilo toliko časa dokler ne zmanka denarja ali pa dajo vsi isto stavoi
    // get_bet naj deluje tako,
    // če player stavi(izenači ali poviša), naj vrne Some<u32>,
    // če pa noče folda naj vrne None,
    // če naredi check naj vrne Some(0)

    // zanka, ki bo šla dokler vsi ne staivijo isto ali pa so vsi brez denarja, ali pa če je samo še en, ki ni foldal
    // sprehoditi se moram po player position, zato bi bilo dobro iz tega narediti iterator

    let start_position = game.position_on_turn.clone();
    let mut players_playing = vec![];
    let mut current_bet: u32 = 0;
    let mut pot: u32 = game.pot;
    loop {
        players_playing.push(game.position_on_turn.clone());
        game.go_to_next_player();
        if game.position_on_turn == start_position {
            break;
        }
    }
    assert!(start_position == game.position_on_turn);
    assert!(!players_playing.is_empty());
    assert!(players_playing[0] == start_position);

    loop {
        let player = game.player_on_turn();
        let bet = get_bet(&player);
        match bet {
            Some(bet) => {
                if bet > current_bet {
                    current_bet = bet;
                }
                if bet + player.current_bet >= current_bet {
                    player.money -= bet; // tukaj naj get_bet function poskrbi da ne bo negativnih vrednosti
                    pot += bet;
                    player.current_bet += bet;
                } else if player.current_bet + player.money < current_bet {
                    player.money -= bet; // tukaj naj get_bet function poskrbi da ne bo negativnih vrednosti
                    pot += bet;
                    player.current_bet += bet;
                } else {
                    continue;
                }
            }
            None => {
                // fold
                player.playing = false;
            }
        }

        game.pot = pot;
        game.go_to_next_player();
        if start_position == game.position_on_turn {
            for pos in players_playing.iter() {
                if game.get_player_from_pos(pos).current_bet != current_bet {
                    continue;
                } else {
                    return;
                }
            }
        }
    }
}
