 use rand::{self, seq::SliceRandom};
use std::{ops::{Deref, FnMut}, default::Default, iter::{Iterator, Map}};
use crate::card::{*, attr::IntegerToEnum};
use super::sim::Simulation;

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
        
        let mut rng = rand::thread_rng();
        cards.shuffle(&mut rng);

        Self {cards}
    }

    pub fn simulate<F, T>(trials: u32, f: F) -> Map<Simulation<Self>, F>
        where F: FnMut(Self) -> T,
    {
        Simulation::<Self>::new(trials).map(f)
    }   
}

impl Deck for ShuffledDeck {
    fn pick(&mut self) -> Option<Card> {
        self.cards.pop()
    }
}

impl Default for ShuffledDeck {
    fn default() -> Self {
        ShuffledDeck::new()
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

    pub fn sum(&self, last_index: usize, ace_val: u32) -> u32 {
        self.cards[..last_index+1].iter().map(|a| a.value().as_num(ace_val)).sum()
    }
}

impl Deck for PlayerHand {
    fn pick(&mut self) -> Option<Card> {
        let mut rng = rand::thread_rng();
        self.cards.shuffle(&mut rng);

        self.cards.pop()
    }
}

impl Deref for PlayerHand {
    type Target = Vec<Card>;

    fn deref(&self) -> &Vec<Card> {
        &self.cards
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dealer_deck_52_cards() {
        let d = ShuffledDeck::new();
        assert_eq!(d.cards.len(), 52);
    }

    #[test]
    fn player_hand_empty() {
        let p = PlayerHand::new();
        assert_eq!(p.cards.len(), 0);
    }
}