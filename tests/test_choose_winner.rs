#[cfg(test)]
mod tests {
    use projektna_prog_2::logic::choose_winner;
	use projektna_prog_2::logic::card::Card;

    use choose_winner::*;
    #[test]
    fn test_is_royal_flush_true() {
        // Assuming `is_royal_flush` takes a hand as input and returns a boolean
        let hand = vec![
            Card::new("10", "H"),
            Card::new("J", "H"),
            Card::new("Q", "H"),
            Card::new("K", "H"),
            Card::new("A", "H"),
        ]; // Example royal flush in hearts
        assert!(is_royal_flush(&hand));
    }

    #[test]
    fn test_is_royal_flush_false() {
        let hand = vec![
            Card::new("9", "H"),
            Card::new("J", "H"),
            Card::new("Q", "H"),
            Card::new("K", "H"),
            Card::new("A", "H"),
        ]; // Not a royal flush
        assert!(!is_royal_flush(&hand));
    }

    #[test]
    fn test_is_royal_flush_mixed_suits() {
        let hand = vec![
            Card::new("10", "H"),
            Card::new("J", "H"),
            Card::new("Q", "H"),
            Card::new("K", "H"),
            Card::new("A", "S"),
        ]; // Mixed suits
        assert!(!is_royal_flush(&hand));
    }

    #[test]
    fn test_is_straight_flush_true() {
        let hand = vec![
            Card::new("9", "H"),
            Card::new("10", "H"),
            Card::new("J", "H"),
            Card::new("Q", "H"),
            Card::new("K", "H"),
        ]; // Example straight flush in hearts
        assert!(is_straight_flush(&hand));
    }

    #[test]
    fn test_is_straight_flush_false() {
        let hand = vec![
            Card::new("9", "H"),
            Card::new("10", "H"),
            Card::new("J", "H"),
            Card::new("Q", "H"),
            Card::new("A", "H"),
        ]; // Not a straight flush
        assert!(!is_straight_flush(&hand));
    }

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
    fn test_is_straight_true() {
        let hand = vec![
            Card::new("9", "H"),
            Card::new("10", "D"),
            Card::new("J", "S"),
            Card::new("Q", "C"),
            Card::new("K", "H"),
        ]; // Straight
        assert!(is_straight(&hand));
    }

    #[test]
    fn test_is_straight_false() {
        let hand = vec![
            Card::new("9", "H"),
            Card::new("10", "D"),
            Card::new("J", "S"),
            Card::new("Q", "C"),
            Card::new("A", "H"),
        ]; // Not a straight
        assert!(!is_straight(&hand));
    }

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
}
