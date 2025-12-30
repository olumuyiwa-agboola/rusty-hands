use strum_macros::EnumIter;

#[derive(Debug, EnumIter, PartialEq, Clone)]
pub enum CardRank {
    Ace,
    King,
    Queen,
    Jack,
    Ten,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two
}

impl CardRank {
    pub fn as_u8(&self) -> u8 {
        match self {
            CardRank::Ace => 12,
            CardRank::King => 11,
            CardRank::Queen => 10,
            CardRank::Jack => 9,
            CardRank::Ten => 8,
            CardRank::Nine => 7,
            CardRank::Eight => 6,
            CardRank::Seven => 5,
            CardRank::Six => 4,
            CardRank::Five => 3,
            CardRank::Four => 2,
            CardRank::Three => 1,
            CardRank::Two => 0,
        }
    }

    pub fn get_rank_from(n: u8) -> Option<CardRank> {
        match n {
            12 => Some(CardRank::Ace),
            11 => Some(CardRank::King),
            10 => Some(CardRank::Queen),
            9 => Some(CardRank::Jack),
            8 => Some(CardRank::Ten),
            7 => Some(CardRank::Nine),
            6 => Some(CardRank::Eight),
            5 => Some(CardRank::Seven),
            4 => Some(CardRank::Six),
            3 => Some(CardRank::Five),
            2 => Some(CardRank::Four),
            1 => Some(CardRank::Three),
            0 => Some(CardRank::Two),
            _ => None
        }
    }
}