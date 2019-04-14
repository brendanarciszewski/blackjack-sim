use rand::prelude::*;
use std::ops::Index;
use crate::card::{*, attr::IntegerToEnum};

pub trait Deck {
    fn pick(&mut self) -> Option<Card>;
}

#[derive(Debug)]
pub struct ShuffledDeck {
    cards: Vec<Card>
}

impl ShuffledDeck {
    pub fn new() -> Self {
        let mut cards = vec![];
        for num_s in 0..4 {
            for num_v in 1..14 {
                let value = attr::Value::get_item(num_v).unwrap();
                let suit = attr::Suit::get_item(num_s).unwrap();
                cards.push(Card::new(value, suit));
            }
        }
        
        let mut rng = thread_rng();
        cards.shuffle(&mut rng);

        Self {cards}
    }
}

impl Deck for ShuffledDeck {
    fn pick(&mut self) -> Option<Card> {
        self.cards.pop()
    }
}


#[derive(Debug)]
pub struct PlayerHand {
    cards: Vec<Card>
}

impl PlayerHand {
    pub fn new() -> Self {
        Self {cards: vec![]}
    }

    pub fn from_card(card: Card) -> Self {
        Self {cards: vec![card]}
    }

    pub fn push(&mut self, card: Card) {
        self.cards.push(card)
    }

    pub fn sum(&self, last_index: usize, ace_val: i32) -> i32 {
        self.cards[..last_index+1].iter().map(|a| a.val(ace_val)).sum()
    }
}

impl Deck for PlayerHand {
    fn pick(&mut self) -> Option<Card> {
        let mut rng = thread_rng();
        self.cards.shuffle(&mut rng);

        self.cards.pop()
    }
}

impl Index<usize> for PlayerHand {
    type Output = Card;

    fn index(&self, i: usize) -> &Card {
        &self.cards[i]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dealer_deck_size() {
        let d = ShuffledDeck::new();
        assert_eq!(d.cards.len(), 52);
    }
}