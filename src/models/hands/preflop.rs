use crate::models::cards::card::Card;
use crate::models::decks::deck::Deck;
use crate::models::enums::hand_rank::HandRank;
use crate::models::enums::card_rank::CardRank;

#[derive(Debug)]
pub struct PreflopHand {
    cards: [Card; 2],
    rank: HandRank
}

impl PreflopHand {
    pub fn cards(&self) -> &[Card; 2] {
        &self.cards
    }

    pub fn rank(&self) -> &HandRank {
        &self.rank
    }

    pub fn from(deck: &mut Deck) -> PreflopHand {
        let cards: [Card; 2] = deck.deal_cards(2).try_into().unwrap();

        let rank: HandRank;
        let hand_is_pair = Self::is_pair([&cards[0], &cards[1]]);

        if hand_is_pair {
            rank = HandRank::Pair
        }else {
            let highest_rank = Self::get_highest_rank([&cards[0].rank(), &cards[1].rank()]);
            rank = HandRank::HighCard(highest_rank);
        }

        PreflopHand { cards, rank }
    }

    fn is_pair(cards: [&Card; 2]) -> bool {
        cards[0].rank() == cards[1].rank()
    }

    fn get_highest_rank(ranks: [&CardRank; 2]) -> CardRank {
        let max_rank= ranks.iter()
                                    .map(|x| x.as_u8()).max();

        match max_rank {
            Some(value) => {
                let rank = CardRank::get_rank_from(value);
                rank.unwrap_or_else(|| CardRank::Two)
            },
            None => CardRank::Two
        }
    }
}