use crate::models::card::Card;
use crate::models::enums::{HandRank, CardRank};

#[derive(Debug)]
pub struct PreflopHand {
    pub cards: [Card; 2],
    pub rank: HandRank
}

impl PreflopHand {
    pub fn from(cards: [Card; 2]) -> PreflopHand {
        let rank: HandRank;
        let hand_is_pair = Self::is_pair([&cards[0], &cards[1]]);

        if hand_is_pair {
            rank = HandRank::Pair
        }else {
            let highest_rank = Self::get_highest_rank([&cards[0].rank, &cards[1].rank]);
            rank = HandRank::HighCard(highest_rank);
        }

        PreflopHand { cards, rank }
    }

    fn is_pair(cards: [&Card; 2]) -> bool {
        cards[0].rank == cards[1].rank
    }

    pub fn get_highest_rank(ranks: [&CardRank; 2]) -> CardRank {
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