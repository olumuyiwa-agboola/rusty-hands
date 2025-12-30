use crate::models::enums::card_suit::CardSuit;
use crate::models::enums::card_rank::CardRank;

#[derive(Debug, Clone, PartialEq)]
pub struct Card {
    pub rank: CardRank,
    pub suit: CardSuit,
    pub symbol: String,
}

impl Card {
    pub fn from(rank: CardRank, suit: CardSuit) -> Card {
        Card {
            symbol: Self::get_symbol(&rank, &suit),
            rank,
            suit,
        }
    }

    fn get_symbol(rank: &CardRank, suit: &CardSuit) -> String {
        let mut symbol = String::from("");
        match rank {
            CardRank::Ace => symbol.push('A'),
            CardRank::Two => symbol.push('2'),
            CardRank::Three => symbol.push('3'),
            CardRank::Four => symbol.push('4'),
            CardRank::Five => symbol.push('5'),
            CardRank::Six => symbol.push('6'),
            CardRank::Seven => symbol.push('7'),
            CardRank::Eight => symbol.push('8'),
            CardRank::Nine => symbol.push('9'),
            CardRank::Ten => symbol.push_str("10"),
            CardRank::Jack => symbol.push('J'),
            CardRank::Queen => symbol.push('Q'),
            CardRank::King => symbol.push('K'),
        };

        match suit {
            CardSuit::Spades => symbol.push_str("♠️"),
            CardSuit::Hearts => symbol.push_str("♥️"),
            CardSuit::Diamonds => symbol.push_str("♦️"),
            CardSuit::Clubs => symbol.push_str("♣️"),
        };

        symbol
    }
}


