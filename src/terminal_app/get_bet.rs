use crate::logic::game::Game;
use std::io;

pub fn get_bet(game: &Game, req_bet: u32) -> Option<u32> {
    // vrne u32, če je pravilno vnešeno in če je prazno vrne None ko hoče foldat
    // ne dovoli staviti več denarja kot ga ima player
    // če je vnešeno karkoli razen u32 ali prazno je treba vnesti ponovno
    // TODO treba še preveriti kakšno stavo mora staviti da ne stavi premalo
    let player = game.player_on_turn_immutable();
    assert!(player.playing); // nedolžno preverjenje ki rešuje use
    println!("Hey {:?}, please enter your bet (empty input to fold and {} to call/check or more to raise): ", player.position, req_bet);
    loop {
        let mut buffer = String::new();
        println!(
            "Player {:?} {:?}, current bet: {}, ",
            player.id, player.position, player.current_bet
        );

        match io::stdin().read_line(&mut buffer) {
            Ok(_) => {
                let input = buffer.trim();
                if input.is_empty() {
                    println!("Player {:?} {:?} folded", player.id, player.position);
                    return None;
                }
                // Poskusi pretvoriti vnos v število
                match input.parse::<u32>() {
                    Ok(bet) => {
                        if bet <= player.chips {
                            if req_bet > bet {
                                println!("Bet too small, if u dont want to fold u need to bet {req_bet}!")
                            } else {
                                println!("{:?} bet: {}", player.position, bet);
                                return Some(bet); // Veljaven vnos, vrni stavo
                            }
                        } else {
                            println!(
                                "Invalid bet! You only have {} money. Try again.",
                                player.chips
                            );
                        }
                    }
                    Err(_) => {
                        println!("Invalid input! Please enter a valid number.");
                    }
                }
            }
            Err(_) => {
                println!("Failed to read input. Please try again.");
            }
        }
    }
}
