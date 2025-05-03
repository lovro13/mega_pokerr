use super::card::Card;
use super::hand_ranking::HandRanking;
// File: src/logic/choose_winner.rs
use super::best_combination::best_combination;
use super::player::Player;
use super::game::Game;
use itertools::Itertools;

pub fn choose_winner(game: &mut Game) -> Vec<&Player> {
    let mut list = Vec::new();
    for player in game.players.iter() {
        let hand_ranking = players_hand_ranking(player, &game.board_cards);
        list.push((player, hand_ranking));
    }

    list.sort_by(|a, b| b.1.cmp(&a.1));
    list.reverse();
    let winning_hand_ranking = list[0].1.clone();
    let mut winners = vec![list.remove(0).0];
    for (player, hand_ranking) in list.iter() {
        if hand_ranking == &winning_hand_ranking {
            winners.push(*player);
        } else {
            break;
        }
    }
    winners
}

pub fn players_hand_ranking(player: &Player, board_cards: &Vec<Card>) -> HandRanking {
    let mut all_cards = board_cards.clone();
    all_cards.push(player.hand_cards.0.clone());
    all_cards.push(player.hand_cards.1.clone());

    let mut combinations = all_cards.into_iter().combinations(5).collect::<Vec<_>>();
    let mut hand_rankings = combinations
        .iter_mut()
        .map(|combination| best_combination(combination))
        .collect::<Vec<_>>();
    hand_rankings.sort();
    hand_rankings.reverse();
    return hand_rankings[0].clone();
}
