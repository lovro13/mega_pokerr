// File: src/logic/choose_winner.rs
use super::player::Player;
use super::round::Game;



pub fn choose_winner(game: &mut Game) -> &Player {
    println!("Hello");
    &game.players[0]
    // TODO TODO TODO
}

