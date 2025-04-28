use projektna_prog_2::logic::card::CardNumber;

#[cfg(test)]
mod tests {
	use super::*;

	fn sort_card_numbers(mut cards: Vec<CardNumber>) -> Vec<CardNumber> {
		cards.sort();
		cards
	}

	#[test]
	fn test_sort_card_numbers_empty() {
		let cards: Vec<CardNumber> = vec![];
		let sorted_cards = sort_card_numbers(cards.clone());
		assert_eq!(sorted_cards, cards);
	}

	#[test]
	fn test_sort_card_numbers_already_sorted() {
		let cards = vec![
			CardNumber::N2,
			CardNumber::N3,
			CardNumber::N4,
			CardNumber::N5,
			CardNumber::N6,
			CardNumber::N7,
			CardNumber::N8,
			CardNumber::N9,
			CardNumber::N10,
			CardNumber::NJ,
			CardNumber::NQ,
			CardNumber::NK,
			CardNumber::NA,
		];
		let sorted_cards = sort_card_numbers(cards.clone());
		assert_eq!(sorted_cards, cards);
	}

	#[test]
	fn test_sort_card_numbers_unsorted() {
		let cards = vec![
			CardNumber::N10,
			CardNumber::N2,
			CardNumber::NA,
			CardNumber::N5,
			CardNumber::NQ,
			CardNumber::N3,
		];
		let sorted_cards = sort_card_numbers(cards.clone());
		assert_eq!(
			sorted_cards,
			vec![
				CardNumber::N2,
				CardNumber::N3,
				CardNumber::N5,
				CardNumber::N10,
				CardNumber::NQ,
				CardNumber::NA
			]
		);
	}

	#[test]
	fn test_sort_card_numbers_with_duplicates() {
		let cards = vec![
			CardNumber::N5,
			CardNumber::N3,
			CardNumber::N5,
			CardNumber::N2,
			CardNumber::N2,
			CardNumber::N10,
		];
		let sorted_cards = sort_card_numbers(cards.clone());
		assert_eq!(
			sorted_cards,
			vec![
				CardNumber::N2,
				CardNumber::N2,
				CardNumber::N3,
				CardNumber::N5,
				CardNumber::N5,
				CardNumber::N10
			]
		);
	}

	#[test]
	fn test_sort_card_numbers_reverse_order() {
		let cards = vec![
			CardNumber::NA,
			CardNumber::NK,
			CardNumber::NQ,
			CardNumber::NJ,
			CardNumber::N10,
			CardNumber::N9,
			CardNumber::N8,
			CardNumber::N7,
			CardNumber::N6,
			CardNumber::N5,
			CardNumber::N4,
			CardNumber::N3,
			CardNumber::N2,
		];
		let sorted_cards = sort_card_numbers(cards.clone());
		assert_eq!(
			sorted_cards,
			vec![
				CardNumber::N2,
				CardNumber::N3,
				CardNumber::N4,
				CardNumber::N5,
				CardNumber::N6,
				CardNumber::N7,
				CardNumber::N8,
				CardNumber::N9,
				CardNumber::N10,
				CardNumber::NJ,
				CardNumber::NQ,
				CardNumber::NK,
				CardNumber::NA
			]
		);
	}
}
