use strum::IntoEnumIterator;
use crate::models::cards::card::Card;
use crate::models::decks::deck::Deck;
use crate::models::enums::card_rank::CardRank;
use crate::models::enums::card_suit::CardSuit;
use crate::models::enums::hand_rank::HandRank;
use crate::models::hands::preflop::PreflopHand;

pub struct FlopHand {
    cards: [Card; 5],
    rank: HandRank
}

impl FlopHand {
    pub fn cards(&self) -> &[Card; 5] {
        &self.cards
    }

    pub fn rank(&self) -> &HandRank {
        &self.rank
    }

    pub fn from(preflop_hand: &PreflopHand, deck: &mut Deck) -> FlopHand {
        let preflop_cards = preflop_hand.cards().clone();
        let flop_cards = deck.deal_cards(3);

        let cards: [Card; 5] = preflop_cards
                                .into_iter()
                                .chain(flop_cards.into_iter())
                                .collect::<Vec<Card>>()
                                .try_into()
                                .unwrap();

        let rank: HandRank = Self::calculate_rank(&cards);

        FlopHand { cards, rank }
    }

    fn calculate_rank(cards: &[Card; 5]) -> HandRank {
        if Self::is_a_flush(cards) && Self::is_a_straight(cards) && Self::highest_card_rank(cards) == CardRank::Ace {
            return HandRank::RoyalFlush
        } else if Self::is_a_flush(cards) && Self::is_a_straight(cards) {
            return HandRank::StraightFlush
        } else if Self::has_a_four_of_a_kind(cards) {
            return HandRank::FourOfAKind
        } else if Self::has_a_three_of_a_kind(cards) && Self::has_a_pair(cards) {
            return HandRank::FullHouse
        } else if Self::is_a_flush(cards) {
            return HandRank::Flush
        } else if Self::is_a_straight(cards) {
            return HandRank::Straight
        } else if Self::has_a_three_of_a_kind(cards) {
            return HandRank::ThreeOfAKind
        } else if Self::has_two_pair(cards) {
            return HandRank::TwoPair
        } else if Self::has_a_pair(cards) {
            return HandRank::Pair
        } else {
            HandRank::HighCard(Self::highest_card_rank(cards))
        }
    }

    fn is_a_flush(cards: &[Card; 5]) -> bool {
        for suit in CardSuit::iter(){
            if cards.iter().all(|card| *(card.suit()) == suit) {
                return true;
            }
        }

        false
    }

    fn is_a_straight(cards: &[Card; 5]) -> bool {
        let card_ranks_as_u8s: [u8; 5] = cards
                                            .iter()
                                            .map(|card| card.rank().as_u8())
                                            .collect::<Vec<u8>>()
                                            .try_into()
                                            .unwrap();

        card_ranks_as_u8s.windows(2).all(|rank_as_u8| rank_as_u8[0] < rank_as_u8[1])
    }

    fn highest_card_rank(cards: &[Card; 5]) -> CardRank {
        let card_ranks_as_u8 = cards
                                            .iter()
                                            .map(|card| card.rank().as_u8())
                                            .collect::<Vec<u8>>();

        CardRank::get_rank_from(*(card_ranks_as_u8.iter().max().unwrap())).unwrap()
    }

    fn has_a_four_of_a_kind(cards: &[Card; 5]) -> bool {
        cards.windows(4).any(|card| card[0].rank() == card[1].rank()
                                                && card[1].rank() == card[2].rank()
                                                && card[2].rank() == card[3].rank())
    }

    fn has_a_three_of_a_kind(cards: &[Card; 5]) -> bool {
        cards.windows(3).any(|card| card[0].rank() == card[1].rank()
                                                && card[1].rank() == card[2].rank())
    }

    fn has_two_pair(cards: &[Card; 5]) -> bool {
        cards.windows(4).any(|card| card[0].rank() == card[1].rank()
                                                && card[2].rank() == card[3].rank())
    }

    fn has_a_pair(cards: &[Card; 5]) -> bool {
        cards.windows(2).any(|card| card[0].rank() == card[1].rank())
    }
}

#[cfg(test)]
mod flop_tests {
    use super::*;
    
    #[test]
    fn negative_test_for_has_three_of_a_kind() {
        let cards = [Card::from(CardRank::Ace, CardSuit::Clubs),
                            Card::from(CardRank::King, CardSuit::Spades),
                            Card::from(CardRank::Ten, CardSuit::Hearts),
                            Card::from(CardRank::Ten, CardSuit::Diamonds),
                            Card::from(CardRank::Five, CardSuit::Diamonds)];

        let has_two_pair = FlopHand::has_two_pair(&cards);

        assert_eq!(has_two_pair, false);
        assert_eq!(FlopHand::calculate_rank(&cards), HandRank::Pair);
    }

    #[test]
    fn positive_test_for_has_two_pair() {
        let cards = [Card::from(CardRank::Ace, CardSuit::Clubs),
                            Card::from(CardRank::Ace, CardSuit::Spades),
                            Card::from(CardRank::Ten, CardSuit::Hearts),
                            Card::from(CardRank::Ten, CardSuit::Diamonds),
                            Card::from(CardRank::Five, CardSuit::Diamonds)];

        let has_two_pair = FlopHand::has_two_pair(&cards);

        assert_eq!(has_two_pair, true);
        assert_eq!(FlopHand::calculate_rank(&cards), HandRank::TwoPair);
    }

    #[test]
    fn negative_test_for_has_two_pair() {
        let cards = [Card::from(CardRank::Ace, CardSuit::Clubs),
                            Card::from(CardRank::King, CardSuit::Spades),
                            Card::from(CardRank::Ten, CardSuit::Hearts),
                            Card::from(CardRank::Ten, CardSuit::Diamonds),
                            Card::from(CardRank::Five, CardSuit::Diamonds)];

        let has_two_pair = FlopHand::has_two_pair(&cards);

        assert_eq!(has_two_pair, false);
        assert_eq!(FlopHand::calculate_rank(&cards), HandRank::Pair);
    }

    #[test]
    fn positive_test_for_has_a_pair() {
        let cards = [Card::from(CardRank::Ace, CardSuit::Clubs),
            Card::from(CardRank::Ace, CardSuit::Spades),
            Card::from(CardRank::Ten, CardSuit::Hearts),
            Card::from(CardRank::Two, CardSuit::Diamonds),
            Card::from(CardRank::Five, CardSuit::Diamonds)];

        let has_a_pair = FlopHand::has_a_pair(&cards);

        assert_eq!(has_a_pair, true);
        assert_eq!(FlopHand::calculate_rank(&cards), HandRank::Pair);
    }

    #[test]
    fn negative_test_for_has_a_pair() {
        let cards = [Card::from(CardRank::Ace, CardSuit::Clubs),
            Card::from(CardRank::King, CardSuit::Spades),
            Card::from(CardRank::Ten, CardSuit::Hearts),
            Card::from(CardRank::Two, CardSuit::Diamonds),
            Card::from(CardRank::Five, CardSuit::Diamonds)];

        let has_a_pair = FlopHand::has_a_pair(&cards);

        assert_eq!(has_a_pair, false);
        assert_eq!(FlopHand::calculate_rank(&cards), HandRank::HighCard(CardRank::Ace));
    }
}