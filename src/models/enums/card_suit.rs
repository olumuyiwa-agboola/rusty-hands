use strum_macros::EnumIter;

#[derive(Debug, EnumIter, Clone, Copy, PartialEq)]
pub enum CardSuit {
    Spades,
    Hearts,
    Diamonds,
    Clubs
}