// File: src/logic/choose_winner.rs
// tests sorting CardNumber

// cargo test --test test_card --features run_with_sdl2
use mega_pokerr::logic::card::Card;
use mega_pokerr::logic::card::CardColor;
use mega_pokerr::logic::card::CardNumber;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_card_numbers_empty() {
        let cards: Vec<CardNumber> = vec![];
        let mut sorted_cards = cards.clone();
        sorted_cards.sort();
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
        let mut sorted_cards = cards.clone();
        sorted_cards.sort();
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
        println!("Unsorted cards: {:?}", cards);
        let mut sorted_cards = cards.clone();
        sorted_cards.sort();
        println!("Sorted cards: {:?}", sorted_cards);
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
        println!("Unsorted cards: {:?}", cards);
        let mut sorted_cards = cards.clone();
        sorted_cards.sort();
        println!("Sorted cards: {:?}", sorted_cards);
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
    fn test_sort_card_vec_empty() {
        let mut cards: Vec<Card> = vec![];
        Card::sort_card_vec(&mut cards);
        assert!(cards.is_empty());
    }

    #[test]
    fn test_sort_card_vec_already_sorted() {
        let mut cards = vec![
            Card {
                color: CardColor::Hearts,
                number: CardNumber::N2,
            },
            Card {
                color: CardColor::Hearts,
                number: CardNumber::N3,
            },
            Card {
                color: CardColor::Hearts,
                number: CardNumber::N4,
            },
        ];
        let expected = vec![
            Card {
                color: CardColor::Hearts,
                number: CardNumber::N2,
            },
            Card {
                color: CardColor::Hearts,
                number: CardNumber::N3,
            },
            Card {
                color: CardColor::Hearts,
                number: CardNumber::N4,
            },
        ];
        Card::sort_card_vec(&mut cards);
        assert_eq!(cards, expected);
    }

    #[test]
    fn test_sort_card_vec_unsorted() {
        let mut cards = vec![
            Card {
                color: CardColor::Hearts,
                number: CardNumber::N4,
            },
            Card {
                color: CardColor::Hearts,
                number: CardNumber::N2,
            },
            Card {
                color: CardColor::Hearts,
                number: CardNumber::N3,
            },
        ];
        let expected = vec![
            Card {
                color: CardColor::Hearts,
                number: CardNumber::N2,
            },
            Card {
                color: CardColor::Hearts,
                number: CardNumber::N3,
            },
            Card {
                color: CardColor::Hearts,
                number: CardNumber::N4,
            },
        ];
        Card::sort_card_vec(&mut cards);
        assert_eq!(cards, expected);
    }

    #[test]
    fn test_sort_card_vec_with_duplicates() {
        let mut cards = vec![
            Card {
                color: CardColor::Hearts,
                number: CardNumber::N3,
            },
            Card {
                color: CardColor::Spades,
                number: CardNumber::N2,
            },
            Card {
                color: CardColor::Diamonds,
                number: CardNumber::N3,
            },
            Card {
                color: CardColor::Clubs,
                number: CardNumber::N5,
            },
            Card {
                color: CardColor::Hearts,
                number: CardNumber::N4,
            },
            Card {
                color: CardColor::Spades,
                number: CardNumber::N5,
            },
        ];
        let expected = vec![
            Card {
                color: CardColor::Spades,
                number: CardNumber::N2,
            },
            Card {
                color: CardColor::Hearts,
                number: CardNumber::N3,
            },
            Card {
                color: CardColor::Diamonds,
                number: CardNumber::N3,
            },
            Card {
                color: CardColor::Hearts,
                number: CardNumber::N4,
            },
            Card {
                color: CardColor::Clubs,
                number: CardNumber::N5,
            },
            Card {
                color: CardColor::Spades,
                number: CardNumber::N5,
            },
        ];
        Card::sort_card_vec(&mut cards);
        assert_eq!(cards, expected);
    }
}
