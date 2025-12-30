use crate::models::cards::card::Card;
use crate::models::enums::card_suit::CardSuit;
use crate::models::enums::card_rank::CardRank;
use crate::models::enums::hand_rank::HandRank;
use crate::models::hands::preflop::PreflopHand;

#[test]
fn a_preflop_hand_is_a_pair_if_the_cards_have_the_same_rank() {
    // Arrange
    let card_1 = Card::from(CardRank::Ace, CardSuit::Hearts);
    let card_2 = Card::from(CardRank::Ace, CardSuit::Spades);

    // Act
    let preflop_hand = PreflopHand::from([card_1, card_2]);

    // Assert
    assert_eq!(preflop_hand.cards[0], Card::from(CardRank::Ace, CardSuit::Hearts));
    assert_eq!(preflop_hand.cards[1], Card::from(CardRank::Ace, CardSuit::Spades));
    assert_eq!(preflop_hand.rank, HandRank::Pair);
}

#[test]
fn a_preflop_hand_is_a_high_card_if_the_cards_have_different_ranks() {
    // Arrange
    let card_1 = Card::from(CardRank::Ace, CardSuit::Hearts);
    let card_2 = Card::from(CardRank::King, CardSuit::Spades);

    // Act
    let preflop_hand = PreflopHand::from([card_1, card_2]);

    // Assert
    assert_eq!(preflop_hand.cards[0], Card::from(CardRank::Ace, CardSuit::Hearts));
    assert_eq!(preflop_hand.cards[1], Card::from(CardRank::King, CardSuit::Spades));
    assert_eq!(preflop_hand.rank, HandRank::HighCard(CardRank::Ace));
}