use crate::models::enums::card_rank::CardRank;

#[derive(PartialEq, Debug)]
pub enum HandRank {
    HighCard(CardRank),
    Pair,
    TwoPair,
    ThreeOfAKind,
    Straight,
    Flush,
    FullHouse,
    FourOfAKind,
    StraightFlush,
    RoyalFlush,
}