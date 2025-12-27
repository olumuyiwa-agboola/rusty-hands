use models::deck::Deck;

mod models;
mod tests;

fn main() {
    let mut deck = Deck::new();
    println!("A deck of cards:");

    println!("Number of cards in deck before dealing: {}", &deck.cards.len());

    let random_card = deck.deal_cards(2);
    for card in random_card {
        println!("A random card from the deck: {}", card.symbol);
    }

    println!("Number of cards in deck after dealing: {}", &deck.cards.len());
}
