
#[cfg(test)]
mod tests {
	use mega_pokerr::logic::best_combination::best_combination;
	use mega_pokerr::logic::card::Card;
	#[test]
	fn test_royal_flush() {
		let mut cards = vec![
			Card::new("10", "S"),
			Card::new("J", "S"),
			Card::new("Q", "S"),
			Card::new("K", "S"),
			Card::new("A", "S"),
		];
		let mut cards2 = vec![
			Card::new("10", "H"),
			Card::new("J", "H"),
			Card::new("Q", "H"),
			Card::new("K", "H"),
			Card::new("A", "C"),
		];
		assert_eq!(
			best_combination(&mut cards) > best_combination(&mut cards2),
			true
		);
	}

	#[test]
	fn test_straight_flush() {
		let mut cards = vec![
			Card::new("9", "S"),
			Card::new("10", "S"),
			Card::new("J", "S"),
			Card::new("Q", "S"),
			Card::new("K", "S"),
		];
		let mut cards2 = vec![
			Card::new("8", "H"),
			Card::new("9", "H"),
			Card::new("10", "H"),
			Card::new("J", "H"),
			Card::new("Q", "H"),
		];
		assert_eq!(
			best_combination(&mut cards) > best_combination(&mut cards2),
			true
		);
	}

	#[test]
	fn test_four_of_a_kind() {
		let mut cards = vec![
			Card::new("A", "S"),
			Card::new("A", "H"),
			Card::new("A", "D"),
			Card::new("A", "C"),
			Card::new("K", "S"),
		];
		let mut cards2 = vec![
			Card::new("K", "S"),
			Card::new("K", "H"),
			Card::new("K", "D"),
			Card::new("K", "C"),
			Card::new("Q", "S"),
		];
		assert_eq!(
			best_combination(&mut cards) > best_combination(&mut cards2),
			true
		);
	}

	#[test]
	fn test_full_house() {
		let mut cards = vec![
			Card::new("A", "S"),
			Card::new("A", "H"),
			Card::new("A", "D"),
			Card::new("K", "S"),
			Card::new("K", "H"),
		];
		let mut cards2 = vec![
			Card::new("Q", "S"),
			Card::new("Q", "H"),
			Card::new("Q", "D"),
			Card::new("J", "S"),
			Card::new("J", "H"),
		];
		assert_eq!(
			best_combination(&mut cards) > best_combination(&mut cards2),
			true
		);
	}

	#[test]
	fn test_flush() {
		let mut cards = vec![
			Card::new("2", "S"),
			Card::new("4", "S"),
			Card::new("6", "S"),
			Card::new("8", "S"),
			Card::new("10", "S"),
		];
		let mut cards2 = vec![
			Card::new("3", "H"),
			Card::new("5", "H"),
			Card::new("7", "H"),
			Card::new("9", "H"),
			Card::new("J", "H"),
		];
		assert_eq!(
			best_combination(&mut cards) > best_combination(&mut cards2),
			false
		);
	}

	#[test]
	fn test_straight() {
		let mut cards = vec![
			Card::new("5", "S"),
			Card::new("6", "H"),
			Card::new("7", "D"),
			Card::new("8", "C"),
			Card::new("9", "S"),
		];
		let mut cards2 = vec![
			Card::new("4", "H"),
			Card::new("5", "D"),
			Card::new("6", "C"),
			Card::new("7", "S"),
			Card::new("8", "H"),
		];
		assert_eq!(
			best_combination(&mut cards) > best_combination(&mut cards2),
			true
		);
	}

	#[test]
	fn test_three_of_a_kind() {
		let mut cards = vec![
			Card::new("A", "S"),
			Card::new("A", "H"),
			Card::new("A", "D"),
			Card::new("K", "S"),
			Card::new("Q", "H"),
		];
		let mut cards2 = vec![
			Card::new("K", "S"),
			Card::new("K", "H"),
			Card::new("K", "D"),
			Card::new("Q", "S"),
			Card::new("J", "H"),
		];
		assert_eq!(
			best_combination(&mut cards) > best_combination(&mut cards2),
			true
		);
	}

	#[test]
	fn test_two_pair() {
		let mut cards = vec![
			Card::new("A", "S"),
			Card::new("A", "H"),
			Card::new("K", "D"),
			Card::new("K", "S"),
			Card::new("Q", "H"),
		];
		let mut cards2 = vec![
			Card::new("Q", "S"),
			Card::new("Q", "H"),
			Card::new("J", "D"),
			Card::new("J", "S"),
			Card::new("10", "H"),
		];
		assert_eq!(
			best_combination(&mut cards) > best_combination(&mut cards2),
			true
		);
	}

	#[test]
	fn test_one_pair() {
		let mut cards = vec![
			Card::new("A", "S"),
			Card::new("A", "H"),
			Card::new("K", "D"),
			Card::new("Q", "S"),
			Card::new("J", "H"),
		];
		let mut cards2 = vec![
			Card::new("K", "S"),
			Card::new("K", "H"),
			Card::new("Q", "D"),
			Card::new("J", "S"),
			Card::new("10", "H"),
		];
		assert_eq!(
			best_combination(&mut cards) > best_combination(&mut cards2),
			true
		);
	}

	#[test]
	fn test_high_card() {
		let mut cards = vec![
			Card::new("A", "S"),
			Card::new("K", "H"),
			Card::new("Q", "D"),
			Card::new("J", "S"),
			Card::new("10", "H"),
		];
		let mut cards2 = vec![
			Card::new("K", "S"),
			Card::new("Q", "H"),
			Card::new("J", "D"),
			Card::new("10", "S"),
			Card::new("9", "H"),
		];
		assert_eq!(
			best_combination(&mut cards) > best_combination(&mut cards2),
			true
		);
	}
}