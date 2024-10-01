use std::vec;

use rand::seq::SliceRandom;
use rand::thread_rng;

#[derive(Debug)]
struct Deck {
    cards: Vec<String>,
}

impl Deck {
    fn new() -> Self {
        let suits = ["Hearts", "Spades", "Diamonds"];
        let values = ["Ace", "Two", "Three"];

        let mut cards = vec![];
        for suit in suits {
            for value in values {
                let card = format!("{} of {}", suit, value);
                cards.push(card);
            }
        }

        Deck { cards }
    }

    fn shuffle(&mut self) {
        let mut rng = thread_rng();
        self.cards.shuffle(&mut rng);
    }
}

fn main() {
    let mut deck = Deck::new();
    deck.shuffle();

    println!("Here your deck: {:#?}", deck);
}
