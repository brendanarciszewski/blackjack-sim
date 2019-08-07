//! Contains useful groups of Cards
use super::sim::Simulation;
use crate::card::*;
use rand::{self, seq::SliceRandom};
use std::{
    convert::TryFrom,
    default::Default,
    iter::{Iterator, Map},
    ops::{Deref, FnMut},
};

///A group of Cards
pub trait Deck {
    fn pick(&mut self) -> Option<Card>;
}

///A deck for the dealer to use
#[derive(Debug)]
pub struct ShuffledDeck {
    cards: Vec<Card>,
}

impl ShuffledDeck {
    ///A shuffled, 52 card, deck
    pub fn new() -> Self {
        let mut cards = vec![];
        for num_s in 0..4 {
            for num_v in 1..14 {
                let value = attr::Value::try_from(num_v).unwrap();
                let suit = attr::Suit::try_from(num_s).unwrap();
                cards.push(Card::new(value, suit));
            }
        }

        let mut rng = rand::thread_rng();
        cards.shuffle(&mut rng);

        Self { cards }
    }

    ///An Iterator that will run `f` on each Card when consumed
    pub fn simulate<F, T>(trials: u32, f: F) -> Map<Simulation<Self>, F>
    where
        F: FnMut(Self) -> T,
    {
        Simulation::<Self>::new(trials).map(f)
    }
}

impl Deck for ShuffledDeck {
    ///The Card at the top of the deck, if it's not empty
    fn pick(&mut self) -> Option<Card> {
        self.cards.pop()
    }
}

impl Default for ShuffledDeck {
    fn default() -> Self {
        ShuffledDeck::new()
    }
}

///Useful to take cards from a deck
#[derive(Debug)]
pub struct PlayerHand {
    cards: Vec<Card>,
}

impl PlayerHand {
    ///An empty PlayerHand
    pub fn new() -> Self {
        Self { cards: vec![] }
    }

    ///A PlayerHand with one card
    pub fn from_card(card: Card) -> Self {
        Self { cards: vec![card] }
    }

    ///Add a card to the hand
    pub fn push(&mut self, card: Card) {
        self.cards.push(card)
    }

    ///Sum the BlackJack value of the hand
    ///```
    ///use blackjack_sim::{deck::PlayerHand, card::{Card, attr}};
    ///let mut player = PlayerHand::new();
    ///player.push(Card::new(attr::Value::Ace, attr::Suit::Heart));
    ///assert_eq!(player.sum(player.len()-1, 1), 1);
    ///```
    pub fn sum(&self, last_index: usize, ace_val: u8) -> u16 {
        self.cards[..=last_index]
            .iter()
            .map(|a| a.value().as_num(ace_val) as u16)
            .sum()
    }
}

impl Deck for PlayerHand {
    ///A random card from the hand
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
