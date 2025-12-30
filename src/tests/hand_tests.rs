use crate::models::card::Card;
use crate::models::hand::PreflopHand;
use crate::models::enums::{CardRank, HandRank, Suit};

#[test]
fn a_preflop_hand_is_a_pair_if_the_cards_have_the_same_rank() {
    // Arrange
    let card_1 = Card::from(CardRank::Ace, Suit::Hearts);
    let card_2 = Card::from(CardRank::Ace, Suit::Spades);

    // Act
    let preflop_hand = PreflopHand::from([card_1, card_2]);

    // Assert
    assert_eq!(preflop_hand.cards[0], Card::from(CardRank::Ace, Suit::Hearts));
    assert_eq!(preflop_hand.cards[1], Card::from(CardRank::Ace, Suit::Spades));
    assert_eq!(preflop_hand.rank, HandRank::Pair);
}

#[test]
fn a_preflop_hand_is_a_high_card_if_the_cards_have_different_ranks() {
    // Arrange
    let card_1 = Card::from(CardRank::Ace, Suit::Hearts);
    let card_2 = Card::from(CardRank::King, Suit::Spades);

    // Act
    let preflop_hand = PreflopHand::from([card_1, card_2]);

    // Assert
    assert_eq!(preflop_hand.cards[0], Card::from(CardRank::Ace, Suit::Hearts));
    assert_eq!(preflop_hand.cards[1], Card::from(CardRank::King, Suit::Spades));
    assert_eq!(preflop_hand.rank, HandRank::HighCard(CardRank::Ace));
}