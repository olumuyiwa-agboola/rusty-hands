use strum::IntoEnumIterator;
use crate::models::card::Card;
use crate::models::deck::Deck;
use crate::models::enums::Suit;
use crate::models::enums::CardRank;

#[test]
fn a_deck_contains_52_cards() {
    // Arrange
    let deck = Deck::new();

    // Assert
    assert_eq!(deck.cards.len(), 52);
    for suit in Suit::iter() {
        let suit_cards: Vec<Card> = deck.cards.iter().filter(|&x| x.suit == suit)
            .cloned()  // Clone the items
            .collect();

        assert_eq!(suit_cards.len(), 13);
    }
    for rank in CardRank::iter() {
        let rank_cards: Vec<Card> = deck.cards.iter().filter(|&x| x.rank == rank)
            .cloned()  // Clone the items
            .collect();

        assert_eq!(rank_cards.len(), 4);
    }
}

#[test]
fn dealing_cards_from_a_deck() {
    let test_cases: Vec<usize> = vec![1, 2, 5];

    for number_of_cards in test_cases {
    // Arrange
    let mut deck = Deck::new();
    let number_of_cards_before_dealing = deck.cards.len();

    let dealt_cards = deck.deal_cards(number_of_cards);
    let number_of_cards_after_dealing = deck.cards.len();

    // Assert
    assert_eq!(number_of_cards_before_dealing, 52);
    assert_eq!(number_of_cards_after_dealing, (52 - number_of_cards));
    assert!(!dealt_cards.iter().any(|x| deck.cards.contains(x)));
    }
}