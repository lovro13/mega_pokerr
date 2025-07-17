use crate::logic::{betting_system::validate_bet, card::Card, constants::*};
use itertools::Itertools;
use crate::logic::best_combination::best_combination;
use crate::logic::hand_ranking::HandRanking;

pub fn rank_cards_preflop(pair_of_cards: Vec<Card>) -> u32 {
    assert!(pair_of_cards.len() == 2);
    let card1 = pair_of_cards[0].clone();
    let card2 = pair_of_cards[1].clone();

    let is_suited = card1.color.clone() == card2.color.clone();
    let (high, low) = if card1.number > card2.number {
        (card1, card2)
    } else {
        (card2, card1)
    };

    let high_index = (14 - high.number.evaluate_to_int()) as usize;
    let low_index = (14 - low.number.evaluate_to_int()) as usize;

    let (i, j) = if is_suited {
        (high_index, low_index)
    } else {
        (low_index, high_index)
    };

    // Tabela pobrana z interneta
    const TABLE: [[u32; 13]; 13] = [
        // Row 0: Ace
        [0, 2, 2, 3, 5, 8, 10, 13, 14, 12, 14, 14, 17],
        // Row 1: King
        [5, 1, 3, 3, 6, 10, 16, 19, 24, 25, 25, 26, 26],
        // Row 2: Queen
        [8, 9, 1, 5, 6, 10, 19, 26, 28, 29, 29, 30, 31],
        // Row 3: Jack
        [12, 14, 15, 2, 6, 11, 17, 27, 33, 35, 37, 37, 38],
        // Row 4: 10
        [18, 20, 22, 21, 4, 10, 16, 25, 31, 40, 40, 41, 41],
        // Row 5: 9
        [32, 35, 36, 34, 31, 7, 17, 24, 29, 38, 47, 47, 49],
        // Row 6: 8
        [39, 50, 53, 48, 43, 42, 9, 21, 27, 27, 33, 40, 54],
        // Row 7: 7
        [45, 57, 66, 64, 59, 55, 52, 12, 25, 28, 37, 45, 56],
        // Row 8: 6
        [51, 60, 71, 80, 74, 68, 61, 57, 16, 27, 29, 38, 49],
        // Row 9: 5
        [44, 63, 75, 82, 89, 83, 73, 65, 58, 20, 28, 32, 39],
        // Row 10: 4
        [46, 67, 76, 85, 90, 95, 88, 78, 70, 62, 23, 36, 41],
        // Row 11: 3
        [49, 67, 77, 86, 92, 96, 98, 93, 81, 72, 76, 23, 46],
        // Row 12: 2
        [54, 69, 79, 87, 94, 97, 99, 100, 95, 84, 86, 91, 24],
    ];

    TABLE[i][j]
}

/// Evaluate the best hand ranking from player and table cards
fn evaluate_hand(player_cards: &(Card, Card), table_cards: &Vec<Card>) -> HandRanking {
    let mut all_cards = table_cards.clone();
    all_cards.push(player_cards.0.clone());
    all_cards.push(player_cards.1.clone());
    let mut best: Option<HandRanking> = None;
    for combo in all_cards.into_iter().combinations(5) {
        let mut combo = combo;
        let hand = best_combination(&mut combo);
        if best.is_none() || hand > best.as_ref().unwrap().clone() {
            best = Some(hand);
        }
    }
    best.unwrap()
}

pub fn make_decision(
    player_cards: &(Card, Card),
    table_cards: &Vec<Card>,
    req_bet: u32,
    players_curr_bet: u32,
    player_chips: u32,
) -> Option<u32> {
    if player_chips == 0 {
        return Some(0);
    }
    // Preflop: use old logic
    if table_cards.is_empty() {
        let hand_cards_vec: Vec<_> = vec![player_cards.0.clone(), player_cards.1.clone()];
        let rank_points = rank_cards_preflop(hand_cards_vec);
        if (rank_points < BOT_RANK_THRESHOLD_LOW) || (rank_points < BOT_RANK_THRESHOLD_HIGH && req_bet <= BOT_BLIND_MULTIPLIER_LOW * BIG_BLIND) {
            if req_bet > player_chips {
                assert!(validate_bet(req_bet, player_chips, player_chips));
                return Some(player_chips);
            }
            if players_curr_bet <= BOT_BLIND_MULTIPLIER_MEDIUM * BIG_BLIND && BOT_BLIND_MULTIPLIER_MEDIUM * BIG_BLIND + req_bet <= player_chips {
                let bet = BOT_BLIND_MULTIPLIER_MEDIUM * BIG_BLIND + req_bet;
                assert!(validate_bet(req_bet, player_chips, bet));
                return Some(bet);
            } else {
                assert!(validate_bet(req_bet, player_chips, req_bet));
                return Some(req_bet);
            }
        } else if rank_points < BOT_RANK_THRESHOLD_MEDIUM && player_chips <= req_bet && players_curr_bet <= BOT_BLIND_MULTIPLIER_LOW * BIG_BLIND {
            if req_bet <= player_chips {
                assert!(validate_bet(req_bet, player_chips, req_bet));
                return Some(req_bet);
            } else {
                assert!(validate_bet(req_bet, player_chips, player_chips));
                return Some(player_chips);
            }
        } else if req_bet == 0 {
            assert!(validate_bet(req_bet, player_chips, 0));
            return Some(0);
        }
    } else {
        // Postflop: use hand strength
        let hand = evaluate_hand(player_cards, table_cards);
        let hand_value = hand.rank_value();
        // Simple logic: bet more with stronger hands
        let bet = if hand_value >= 7 { // Full house or better
            // Go all-in if strong
            player_chips
        } else if hand_value >= 5 { // Straight or flush
            // Raise moderately
            let raise = (req_bet + BIG_BLIND * 2).min(player_chips);
            raise
        } else if hand_value >= 2 { // Pair or two pair or three of a kind
            // Call
            req_bet.min(player_chips)
        } else {
            // Weak hand: only call/check
            req_bet.min(player_chips)
        };
        // Always return a valid bet
        let valid_bet = if bet < req_bet {
            // If can't call, go all-in if possible, else fold (but never return None)
            if player_chips >= req_bet {
                req_bet
            } else {
                player_chips
            }
        } else if bet > player_chips {
            player_chips
        } else {
            bet
        };
        assert!(validate_bet(req_bet, player_chips, valid_bet));
        return Some(valid_bet);
    }
    // Fallback: always call or go all-in
    let fallback_bet = if player_chips >= req_bet { req_bet } else { player_chips };
    assert!(validate_bet(req_bet, player_chips, fallback_bet));
    Some(fallback_bet)
}
