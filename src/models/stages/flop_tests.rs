use crate::models::decks::deck::Deck;
use crate::models::stages::flop::Flop;

#[test]
fn a_flop_stage_takes_three_cards_from_a_deck() {
    // Arrange
    let mut deck = Deck::new();

    // Act
    let flop = Flop::from(&mut deck);

    // Assert
    assert_eq!(flop.cards.len(), 3);
    assert_eq!(deck.cards.len(), 49);
    assert!(!flop.cards.iter().any(|x| deck.cards.contains(x)));
}