use crate::logic::game;

pub fn print_game_info(game: &game::Game) {
    println!("============GAME INFO===========");
    println!("Round: {}", game.round_number);
    println!("Current Street: {:?}", game.street);
    println!("Pot: {}", game.pot);
    
    for player in game.players.iter() {
        println!("===========Player {:?}==========", player.name);
        println!("{:?} money: {}", player.name, player.chips);
        println!("{:?} position: {:?}", player.name, player.position);
        println!("{:?} cards: {}, {}", player.name, player.hand_cards.0, player.hand_cards.1);
        println!("{:?} current bet: {}", player.name, player.current_bet);
        println!("{:?} playing: {}", player.name, player.playing);
        println!();
    }
    
}
