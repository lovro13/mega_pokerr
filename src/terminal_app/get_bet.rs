use crate::logic::player;
use std::io;

pub fn get_bet(player: &player::Player) -> Option<u32> {
    // vrne u32, če je pravilno vnešeno in None če hoče foldat
    println!("please enter your bet (0 to check, empty input to skip): ");
    loop {
        let mut buffer = String::new();
        println!(
            "Player {:?} {:?}, current bet: {}, ",
            player.name, player.position, player.current_bet
        );

        match io::stdin().read_line(&mut buffer) {
            Ok(_) => {
                let input = buffer.trim();
                if input.is_empty() {
                    return None;
                }
                // Poskusi pretvoriti vnos v število
                match input.parse::<u32>() {
                    Ok(bet) => {
                        if bet <= player.money {
                            println!("{:?} bet: {}", player.position, bet);
                            return Some(bet); // Veljaven vnos, vrni stavo
                        } else {
                            println!(
                                "Invalid bet! You only have {} money. Try again.",
                                player.money
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
