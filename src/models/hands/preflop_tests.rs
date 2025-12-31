use crate::models::decks::deck::Deck;
use crate::models::hands::preflop::PreflopHand;

#[test]
fn a_preflop_hand_is_consists_of_two_cards_taken_from_a_deck() {
    // Arrange
    let mut deck = Deck::new();

    // Act
    let preflop_hand = PreflopHand::from(&mut deck);

    // Assert
    assert_eq!(deck.cards().len(), 50);
    assert_eq!(preflop_hand.cards().len(), 2);
    assert!(!preflop_hand.cards().iter().any(|x| deck.cards().contains(x)));
}