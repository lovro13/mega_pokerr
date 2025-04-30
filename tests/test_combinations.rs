// cargo test --test test_card --features run_with_sdl2

#[cfg(test)]
mod tests {
    use mega_pokerr::logic::card::Card;
    use mega_pokerr::logic::combinations;

    use combinations::{
        is_flush, is_four_of_a_kind, is_full_house, is_one_pair, is_royal_flush, is_straight,
        is_straight_flush, is_three_of_a_kind, is_two_pair,
    };
    #[test]

    // ROYAL FLUSH =============================================================
    fn test_is_royal_flush_true() {
        // Assuming `is_royal_flush` takes a hand as input and returns a boolean
        let mut hand = vec![
            Card::new("10", "H"),
            Card::new("J", "H"),
            Card::new("Q", "H"),
            Card::new("K", "H"),
            Card::new("A", "H"),
        ]; // Example royal flush in hearts
        assert!(is_royal_flush(&mut hand));
    }

    #[test]
    fn test_is_royal_flush_false() {
        let mut hand = vec![
            Card::new("9", "H"),
            Card::new("J", "H"),
            Card::new("Q", "H"),
            Card::new("K", "H"),
            Card::new("A", "H"),
        ]; // Not a royal flush
        assert!(!is_royal_flush(&mut hand));
    }

    #[test]
    fn test_is_royal_flush_mixed_suits() {
        let mut hand = vec![
            Card::new("10", "H"),
            Card::new("J", "H"),
            Card::new("Q", "H"),
            Card::new("K", "H"),
            Card::new("A", "S"),
        ]; // Mixed suits
        assert!(!is_royal_flush(&mut hand));
    }
    // =========================================================================
    // STRAIGHT FLUSH ==========================================================
    #[test]
    fn test_is_straight_flush_true() {
        let mut hand = vec![
            Card::new("9", "H"),
            Card::new("10", "H"),
            Card::new("J", "H"),
            Card::new("Q", "H"),
            Card::new("K", "H"),
        ]; // Example straight flush in hearts
        assert!(is_straight_flush(&mut hand));
    }

    #[test]
    fn test_is_straight_flush_false() {
        let mut hand = vec![
            Card::new("9", "H"),
            Card::new("10", "H"),
            Card::new("J", "H"),
            Card::new("Q", "H"),
            Card::new("A", "H"),
        ]; // Not a straight flush
        assert!(!is_straight_flush(&mut hand));
    }
    #[test]
    fn test_is_straight_flush_true_ace_low() {
        let mut hand = vec![
            Card::new("A", "H"),
            Card::new("2", "H"),
            Card::new("3", "H"),
            Card::new("4", "H"),
            Card::new("5", "H"),
        ]; // Straight flush with Ace as low
        assert!(is_straight_flush(&mut hand));
    }

    #[test]
    fn test_is_straight_flush_false_mixed_suits() {
        let mut hand = vec![
            Card::new("9", "H"),
            Card::new("10", "H"),
            Card::new("J", "H"),
            Card::new("Q", "H"),
            Card::new("K", "S"),
        ]; // Not a straight flush due to mixed suits
        assert!(!is_straight_flush(&mut hand));
    }
    // =========================================================================
    // FOUR OF A KIND ==========================================================
    #[test]
    fn test_is_four_of_kind_true() {
        let hand = vec![
            Card::new("A", "H"),
            Card::new("A", "D"),
            Card::new("A", "S"),
            Card::new("A", "C"),
            Card::new("K", "H"),
        ]; // Four of a kind
        assert!(is_four_of_a_kind(&hand));
    }

    #[test]
    fn test_is_four_of_kind_false() {
        let hand = vec![
            Card::new("A", "H"),
            Card::new("A", "D"),
            Card::new("A", "S"),
            Card::new("K", "C"),
            Card::new("K", "H"),
        ]; // Not four of a kind
        assert!(!is_four_of_a_kind(&hand));
    }
    #[test]
    fn test_is_four_of_kind_true_with_low_cards() {
        let hand = vec![
            Card::new("2", "H"),
            Card::new("2", "D"),
            Card::new("2", "S"),
            Card::new("2", "C"),
            Card::new("3", "H"),
        ]; // Four of a kind with low cards
        assert!(is_four_of_a_kind(&hand));
    }

    #[test]
    fn test_is_four_of_kind_false_with_three_of_a_kind() {
        let hand = vec![
            Card::new("7", "H"),
            Card::new("7", "D"),
            Card::new("7", "S"),
            Card::new("8", "C"),
            Card::new("9", "H"),
        ]; // Not four of a kind, only three of a kind
        assert!(!is_four_of_a_kind(&hand));
    }

    // =========================================================================
    // FULL HOUSE ==============================================================
    #[test]
    fn test_is_full_house_true() {
        let hand = vec![
            Card::new("A", "H"),
            Card::new("A", "D"),
            Card::new("A", "S"),
            Card::new("K", "C"),
            Card::new("K", "H"),
        ]; // Full house
        assert!(is_full_house(&hand));
    }

    #[test]
    fn test_is_full_house_false() {
        let hand = vec![
            Card::new("A", "H"),
            Card::new("A", "D"),
            Card::new("K", "S"),
            Card::new("K", "C"),
            Card::new("Q", "H"),
        ]; // Not a full house
        assert!(!is_full_house(&hand));
    }
    #[test]
    fn test_is_full_house_true_low_cards() {
        let hand = vec![
            Card::new("2", "H"),
            Card::new("2", "D"),
            Card::new("2", "S"),
            Card::new("3", "C"),
            Card::new("3", "H"),
        ]; // Full house with low cards
        assert!(is_full_house(&hand));
    }

    #[test]
    fn test_is_full_house_false_four_of_a_kind() {
        let hand = vec![
            Card::new("A", "H"),
            Card::new("A", "D"),
            Card::new("A", "S"),
            Card::new("A", "C"),
            Card::new("K", "H"),
        ]; // Four of a kind is not a full house
        assert!(!is_full_house(&hand));
    }
    // =========================================================================
    // FLUSH ===================================================================
    #[test]
    fn test_is_flush_true() {
        let hand = vec![
            Card::new("2", "H"),
            Card::new("5", "H"),
            Card::new("9", "H"),
            Card::new("J", "H"),
            Card::new("K", "H"),
        ]; // Flush in hearts
        assert!(is_flush(&hand));
    }

    #[test]
    fn test_is_flush_false() {
        let hand = vec![
            Card::new("2", "H"),
            Card::new("5", "H"),
            Card::new("9", "H"),
            Card::new("J", "H"),
            Card::new("K", "S"),
        ]; // Not a flush
        assert!(!is_flush(&hand));
    }
    #[test]
    fn test_is_flush_true_low_cards() {
        let hand = vec![
            Card::new("2", "D"),
            Card::new("3", "D"),
            Card::new("4", "D"),
            Card::new("5", "D"),
            Card::new("7", "D"),
        ]; // Flush in diamonds with low cards
        assert!(is_flush(&hand));
    }

    #[test]
    fn test_is_flush_false_one_off() {
        let hand = vec![
            Card::new("2", "H"),
            Card::new("5", "H"),
            Card::new("9", "H"),
            Card::new("J", "H"),
            Card::new("K", "H"),
        ]; // Flush in hearts
        assert!(is_flush(&hand));
    }

    // =========================================================================
    // STRAIGHT ================================================================
    #[test]
    fn next_ace_is_two() {
        let card = Card::new("A", "H");
        let next_card = card.next_in_straight();
        assert_eq!(next_card, Card::new("2", "H"));
    }

    #[test]
    fn test_is_straight_true_ace_low() {
        let mut hand = vec![
            Card::new("A", "H"),
            Card::new("2", "D"),
            Card::new("3", "S"),
            Card::new("4", "C"),
            Card::new("5", "H"),
        ]; // Straight with Ace as low
        assert!(is_straight(&mut hand));
    }

    #[test]
    fn test_is_straight_false_duplicate_cards() {
        let mut hand = vec![
            Card::new("9", "H"),
            Card::new("10", "D"),
            Card::new("J", "S"),
            Card::new("J", "C"),
            Card::new("K", "H"),
        ]; // Not a straight due to duplicate cards
        assert!(!is_straight(&mut hand));
    }

    #[test]
    fn test_is_straight_false() {
        let mut hand = vec![
            Card::new("9", "H"),
            Card::new("10", "D"),
            Card::new("J", "S"),
            Card::new("Q", "C"),
            Card::new("A", "H"),
        ]; // Not a straight
        assert!(!is_straight(&mut hand));
    }

    // =========================================================================
    // THREE OF A KIND =========================================================
    #[test]
    fn test_is_three_of_kind_true() {
        let hand = vec![
            Card::new("A", "H"),
            Card::new("A", "D"),
            Card::new("A", "S"),
            Card::new("K", "C"),
            Card::new("Q", "H"),
        ]; // Three of a kind
        assert!(is_three_of_a_kind(&hand));
    }

    #[test]
    fn test_is_three_of_kind_false() {
        let hand = vec![
            Card::new("A", "H"),
            Card::new("A", "D"),
            Card::new("K", "S"),
            Card::new("K", "C"),
            Card::new("Q", "H"),
        ]; // Not three of a kind
        assert!(!is_three_of_a_kind(&hand));
    }

    #[test]
    fn test_is_three_of_kind_true_low_cards() {
        let hand = vec![
            Card::new("2", "H"),
            Card::new("2", "D"),
            Card::new("2", "S"),
            Card::new("K", "C"),
            Card::new("Q", "H"),
        ]; // Three of a kind with low cards
        assert!(is_three_of_a_kind(&hand));
    }

    #[test]
    fn test_is_three_of_kind_false_two_pair() {
        let hand = vec![
            Card::new("A", "H"),
            Card::new("A", "D"),
            Card::new("K", "S"),
            Card::new("K", "C"),
            Card::new("Q", "H"),
        ]; // Two pair is not three of a kind
        assert!(!is_three_of_a_kind(&hand));
    }
    // =========================================================================
    // TWO PAIR ================================================================
    #[test]
    fn test_is_two_pair_true() {
        let hand = vec![
            Card::new("A", "H"),
            Card::new("A", "D"),
            Card::new("K", "S"),
            Card::new("K", "C"),
            Card::new("Q", "H"),
        ]; // Two pair
        assert!(is_two_pair(&hand));
    }

    #[test]
    fn test_is_two_pair_false() {
        let hand = vec![
            Card::new("A", "H"),
            Card::new("A", "D"),
            Card::new("K", "S"),
            Card::new("Q", "C"),
            Card::new("J", "H"),
        ]; // Not two pair
        assert!(!is_two_pair(&hand));
    }
    #[test]
    fn test_is_two_pair_true_low_cards() {
        let hand = vec![
            Card::new("2", "H"),
            Card::new("2", "D"),
            Card::new("3", "S"),
            Card::new("3", "C"),
            Card::new("Q", "H"),
        ]; // Two pair with low cards
        assert!(is_two_pair(&hand));
    }

    #[test]
    fn test_is_two_pair_false_full_house() {
        let hand = vec![
            Card::new("A", "H"),
            Card::new("A", "D"),
            Card::new("K", "S"),
            Card::new("K", "C"),
            Card::new("K", "H"),
        ]; // Full house is not two pair
        assert!(!is_two_pair(&hand));
    }
    // =========================================================================
    // ONE PAIR ================================================================
    #[test]
    fn test_is_pair_true() {
        let hand = vec![
            Card::new("A", "H"),
            Card::new("A", "D"),
            Card::new("K", "S"),
            Card::new("Q", "C"),
            Card::new("J", "H"),
        ]; // One pair
        assert!(is_one_pair(&hand));
    }

    #[test]
    fn test_is_pair_false() {
        let hand = vec![
            Card::new("A", "H"),
            Card::new("K", "D"),
            Card::new("Q", "S"),
            Card::new("J", "C"),
            Card::new("10", "H"),
        ]; // No pair
        assert!(!is_one_pair(&hand));
    }

    #[test]
    fn test_is_pair_true_2() {
        let hand = vec![
            Card::new("7", "H"),
            Card::new("7", "D"),
            Card::new("2", "S"),
            Card::new("5", "C"),
            Card::new("9", "H"),
        ];
        assert!(is_one_pair(&hand));
    }

    #[test]
    fn test_is_pair_false_with_three_of_a_kind() {
        let hand = vec![
            Card::new("7", "H"),
            Card::new("7", "D"),
            Card::new("7", "S"),
            Card::new("5", "C"),
            Card::new("9", "H"),
        ];
        assert!(!is_one_pair(&hand));
    }

    #[test]
    fn test_is_pair_true_low_cards() {
        let hand = vec![
            Card::new("2", "H"),
            Card::new("2", "D"),
            Card::new("3", "S"),
            Card::new("4", "C"),
            Card::new("5", "H"),
        ]; // One pair with low cards
        assert!(is_one_pair(&hand));
    }

    #[test]
    fn test_is_pair_false_two_pair() {
        let hand = vec![
            Card::new("A", "H"),
            Card::new("A", "D"),
            Card::new("K", "S"),
            Card::new("K", "C"),
            Card::new("Q", "H"),
        ]; // pair bo true tudi, če imamo 2 pair
        assert!(is_one_pair(&hand));
    }
}
