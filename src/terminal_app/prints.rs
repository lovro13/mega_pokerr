use crate::logic::round;

pub fn print_round(round: &round::Game) {
    for player in round.players.iter() {
        println!("Player name: {:?}", player.name);
        println!("Player money: {}", player.money);
        println!("Player position: {:?}", player.position);
        println!("Player cards: {}, {}", player.cards.0, player.cards.1);
        println!("Player current bet: {}", player.current_bet);
        println!("Player playing: {}", player.playing);
        println!();
    }
    println!("=======Cards were dealt!========");
    println!("Street: {:?}", round.street);
    println!("Pot: {}", round.pot);
    println!();
}
