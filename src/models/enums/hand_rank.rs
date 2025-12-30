use crate::models::enums::card_rank::CardRank;

#[derive(Debug, PartialEq)]
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