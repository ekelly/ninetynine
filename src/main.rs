extern crate cards;

use cards::deck::Deck;
use cards::card::{Card, Rank};

fn main() {
    println!("Hello, world!");

    let mut deck = Deck::new_shuffled_subset(Rank::Six, Rank::Ace);
    let card_result = deck.deal();

    println!("First card: {}", card_result.unwrap());
}
