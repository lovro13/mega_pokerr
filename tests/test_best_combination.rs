#[cfg(test)]
mod tests {
    use mega_pokerr::logic::best_combination::{best_combination, HandRanking};
    use mega_pokerr::logic::card::{Card, CardNumber};

	#[test]
	fn test_royal_flush_alternate() {
		let mut cards = vec![
			Card::new("10", "S"),
			Card::new("J", "S"),
			Card::new("Q", "S"),
			Card::new("K", "S"),
			Card::new("A", "S"),
		];
		assert_eq!(
			best_combination(&mut cards),
			HandRanking::RoyalFlush(vec![])
		);
	}

	#[test]
	fn test_straight_flush_alternate() {
		let mut cards = vec![
			Card::new("8", "C"),
			Card::new("9", "C"),
			Card::new("10", "C"),
			Card::new("J", "C"),
			Card::new("Q", "C"),
		];
		assert_eq!(
			best_combination(&mut cards),
			HandRanking::StraightFlush(vec![CardNumber::NQ])
		);
	}

	#[test]
	fn test_four_of_a_kind_alternate() {
		let mut cards = vec![
			Card::new("7", "H"),
			Card::new("7", "D"),
			Card::new("7", "S"),
			Card::new("7", "C"),
			Card::new("A", "H"),
		];
		assert_eq!(
			best_combination(&mut cards),
			HandRanking::FourOfAKind(vec![CardNumber::N7, CardNumber::NA])
		);
	}

	#[test]
	fn test_full_house_alternate() {
		let mut cards = vec![
			Card::new("J", "H"),
			Card::new("J", "D"),
			Card::new("J", "S"),
			Card::new("Q", "H"),
			Card::new("Q", "D"),
		];
		assert_eq!(
			best_combination(&mut cards),
			HandRanking::FullHouse(vec![CardNumber::NJ, CardNumber::NQ])
		);
	}

	#[test]
	fn test_flush_alternate() {
		let mut cards = vec![
			Card::new("3", "D"),
			Card::new("6", "D"),
			Card::new("8", "D"),
			Card::new("J", "D"),
			Card::new("Q", "D"),
		];
		assert_eq!(
			best_combination(&mut cards),
			HandRanking::Flush(vec![CardNumber::NQ])
		);
	}

	#[test]
	fn test_straight_alternate() {
		let mut cards = vec![
			Card::new("4", "H"),
			Card::new("5", "D"),
			Card::new("6", "S"),
			Card::new("7", "C"),
			Card::new("8", "H"),
		];
		assert_eq!(
			best_combination(&mut cards),
			HandRanking::Straight(vec![CardNumber::N8])
		);
	}

	#[test]
	fn test_three_of_a_kind_alternate() {
		let mut cards = vec![
			Card::new("K", "H"),
			Card::new("K", "D"),
			Card::new("K", "S"),
			Card::new("10", "C"),
			Card::new("J", "H"),
		];
		assert_eq!(
			best_combination(&mut cards),
			HandRanking::ThreeOfAKind(vec![CardNumber::NK, CardNumber::NJ, CardNumber::N10])
		);
	}

	#[test]
	fn test_two_pair_alternate() {
		let mut cards = vec![
			Card::new("8", "H"),
			Card::new("8", "D"),
			Card::new("Q", "S"),
			Card::new("Q", "C"),
			Card::new("5", "H"),
		];
		assert_eq!(
			best_combination(&mut cards),
			HandRanking::TwoPair(vec![CardNumber::NQ, CardNumber::N8, CardNumber::N5])
		);
	}

	#[test]
	fn test_one_pair_alternate() {
		let mut cards = vec![
			Card::new("9", "H"),
			Card::new("9", "D"),
			Card::new("7", "S"),
			Card::new("6", "C"),
			Card::new("5", "H"),
		];
		assert_eq!(
			best_combination(&mut cards),
			HandRanking::OnePair(vec![CardNumber::N9, CardNumber::N7, CardNumber::N6, CardNumber::N5])
		);
	}

	#[test]
	fn test_high_card_alternate() {
		let mut cards = vec![
			Card::new("3", "H"),
			Card::new("4", "D"),
			Card::new("6", "S"),
			Card::new("8", "C"),
			Card::new("10", "H"),
		];
		assert_eq!(
			best_combination(&mut cards),
			HandRanking::HighCard(vec![CardNumber::N10, CardNumber::N8, CardNumber::N6, CardNumber::N4, CardNumber::N3])
		);
	}
    #[test]
    fn test_royal_flush() {
		let mut cards = vec![
			Card::new("10", "H"),
			Card::new("J", "H"),
			Card::new("Q", "H"),
			Card::new("K", "H"),
			Card::new("A", "H"),
		];
        assert_eq!(
            best_combination(&mut cards),
            HandRanking::RoyalFlush(vec![])
        );
    }
	#[test]
	fn test_straight_flush() {
		let mut cards = vec![
			Card::new("9", "H"),
			Card::new("10", "H"),
			Card::new("J", "H"),
			Card::new("Q", "H"),
			Card::new("K", "H"),
		];
		assert_eq!(
			best_combination(&mut cards),
			HandRanking::StraightFlush(vec![CardNumber::NK])
		);
	}

	#[test]
	fn test_four_of_a_kind() {
		let mut cards = vec![
			Card::new("9", "H"),
			Card::new("9", "D"),
			Card::new("9", "S"),
			Card::new("9", "C"),
			Card::new("K", "H"),
		];
		assert_eq!(
			best_combination(&mut cards),
			HandRanking::FourOfAKind(vec![CardNumber::N9, CardNumber::NK])
		);
	}

	#[test]
	fn test_full_house() {
		let mut cards = vec![
			Card::new("10", "H"),
			Card::new("10", "D"),
			Card::new("10", "S"),
			Card::new("K", "H"),
			Card::new("K", "D"),
		];
		assert_eq!(
			best_combination(&mut cards),
			HandRanking::FullHouse(vec![CardNumber::N10, CardNumber::NK])
		);
	}

	#[test]
	fn test_flush() {
		let mut cards = vec![
			Card::new("2", "H"),
			Card::new("5", "H"),
			Card::new("9", "H"),
			Card::new("J", "H"),
			Card::new("K", "H"),
		];
		assert_eq!(
			best_combination(&mut cards),
			HandRanking::Flush(vec![CardNumber::NK])
		);
	}

	#[test]
	fn test_straight() {
		let mut cards = vec![
			Card::new("6", "H"),
			Card::new("7", "D"),
			Card::new("8", "S"),
			Card::new("9", "C"),
			Card::new("10", "H"),
		];
		assert_eq!(
			best_combination(&mut cards),
			HandRanking::Straight(vec![CardNumber::N10])
		);
	}

	#[test]
	fn test_three_of_a_kind() {
		let mut cards = vec![
			Card::new("Q", "H"),
			Card::new("Q", "D"),
			Card::new("Q", "S"),
			Card::new("9", "C"),
			Card::new("K", "H"),
		];
		assert_eq!(
			best_combination(&mut cards),
			HandRanking::ThreeOfAKind(vec![CardNumber::NQ, CardNumber::NK, CardNumber::N9])
		);
	}

	#[test]
	fn test_two_pair() {
		let mut cards = vec![
			Card::new("J", "H"),
			Card::new("J", "D"),
			Card::new("K", "S"),
			Card::new("K", "C"),
			Card::new("9", "H"),
		];
		assert_eq!(
			best_combination(&mut cards),
			HandRanking::TwoPair(vec![CardNumber::NK, CardNumber::NJ, CardNumber::N9])
		);
	}

	#[test]
	fn test_one_pair() {
		let mut cards = vec![
			Card::new("A", "H"),
			Card::new("A", "D"),
			Card::new("10", "S"),
			Card::new("9", "C"),
			Card::new("K", "H"),
		];
		assert_eq!(
			best_combination(&mut cards),
			HandRanking::OnePair(vec![CardNumber::NA, CardNumber::NK, CardNumber::N10, CardNumber::N9])
		);
	}

	#[test]
	fn test_high_card() {
		let mut cards = vec![
			Card::new("2", "H"),
			Card::new("5", "D"),
			Card::new("9", "S"),
			Card::new("J", "C"),
			Card::new("K", "H"),
		];
		assert_eq!(
			best_combination(&mut cards),
			HandRanking::HighCard(vec![CardNumber::NK, CardNumber::NJ, CardNumber::N9, CardNumber::N5, CardNumber::N2])
		);
	}
}
