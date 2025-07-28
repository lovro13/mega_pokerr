use crate::logic::{betting_system::validate_bet, card::Card, constants::*};

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

pub fn make_decision(
    player_cards: &(Card, Card),
    _table_cards: &Vec<Card>, // v planu še uporabit
    req_bet: u32,
    players_curr_bet: u32, // samo za to da nau preveč stavu in da enkrat neha
    player_chips: u32,
) -> Option<u32> {
    if player_chips == 0 {
        return Some(0);
    }
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
            Some(bet)
        } else {
            assert!(validate_bet(req_bet, player_chips, req_bet));
            Some(req_bet)
        }
    } else if rank_points < BOT_RANK_THRESHOLD_MEDIUM && player_chips <= req_bet && players_curr_bet <= BOT_BLIND_MULTIPLIER_LOW * BIG_BLIND {
        if req_bet <= player_chips {
            assert!(validate_bet(req_bet, player_chips, req_bet));
            Some(req_bet)
        } else {
            assert!(validate_bet(req_bet, player_chips, player_chips));
            Some(player_chips)
        }
    } else if req_bet == 0 {
        assert!(validate_bet(req_bet, player_chips, 0));
        return Some(0);
    } else {
        None
    }
}
