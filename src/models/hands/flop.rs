use crate::models::cards::card::Card;
use crate::models::enums::hand_rank::HandRank;

#[derive(Debug)]
pub struct FlopHand {
    pub cards: [Card; 5],
    pub rank: HandRank
}

impl FlopHand {
    // pub fn from(cards: [Card; 5]) -> FlopHand {
    //     let rank: HandRank;
    //     let hand_is_pair = Self::is_pair([&cards[0], &cards[1]]);
    //
    //     if hand_is_pair {
    //         rank = HandRank::Pair
    //     }else {
    //         let highest_rank = Self::get_highest_rank([&cards[0].rank, &cards[1].rank]);
    //         rank = HandRank::HighCard(highest_rank);
    //     }
    //
    //     FlopHand { cards, rank }
    // }
}