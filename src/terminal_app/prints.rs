use crate::logic::game;

pub fn print_game_info(game: &game::Game) {
    println!("============GAME INFO===========");
    println!("Round: {}", game.round_number);
    println!("Current Street: {:?}", game.street);
    println!("Pot: {}", game.pot);
    
    for player in game.players.iter() {
        println!("===========Player {:?}==========", player.id);
        println!("{:?} money: {}", player.id, player.chips);
        println!("{:?} position: {:?}", player.id, player.position);
        println!("{:?} cards: {}, {}", player.id, player.hand_cards.0, player.hand_cards.1);
        println!("{:?} current bet: {}", player.id, player.current_bet);
        println!("{:?} playing: {}", player.id, player.playing);
        println!();
    }
    
}
