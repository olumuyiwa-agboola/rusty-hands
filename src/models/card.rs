use crate::models::enums::{Rank, Suit};

#[derive(Debug)]
pub struct Card {
    pub rank: Rank,
    pub suit: Suit,
    pub symbol: String,
}

impl Card {
    pub fn from(rank: Rank, suit: Suit) -> Card {
        Card {
            symbol: Self::get_symbol(&rank, &suit),
            rank,
            suit,
        }
    }

    fn get_symbol(rank: &Rank, suit: &Suit) -> String {
        let mut symbol = String::from("");
        match rank {
            Rank::Ace => symbol.push('A'),
            Rank::Two => symbol.push('2'),
            Rank::Three => symbol.push('3'),
            Rank::Four => symbol.push('4'),
            Rank::Five => symbol.push('5'),
            Rank::Six => symbol.push('6'),
            Rank::Seven => symbol.push('7'),
            Rank::Eight => symbol.push('8'),
            Rank::Nine => symbol.push('9'),
            Rank::Ten => symbol.push_str("10"),
            Rank::Jack => symbol.push('J'),
            Rank::Queen => symbol.push('Q'),
            Rank::King => symbol.push('K'),
        };

        match suit {
            Suit::Spades => symbol.push_str("♠️"),
            Suit::Hearts => symbol.push_str("♥️"),
            Suit::Diamonds => symbol.push_str("♦️"),
            Suit::Clubs => symbol.push_str("♣️"),
        };

        symbol
    }
}


