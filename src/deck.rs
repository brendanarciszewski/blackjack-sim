use rand::prelude::*;
use std::ops::Index;

#[derive(Debug, PartialEq)]
enum Suit {
    Heart,
    Diamond,
    Spade,
    Club
}

#[derive(Debug, PartialEq, Clone)]
enum Value {
    Ace = 1,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King
}

impl Value {
    fn val(&self, ace_val: i32) -> i32 {
        match self {
            Value::Ace => ace_val,
            Value::Jack | Value::Queen | Value::King => 10 as i32,
            x => x.clone() as i32 
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Card {
    value: Value,
    suit: Suit
}

impl Card {
    pub fn val(&self, ace_val: i32) -> i32 {
        self.value.val(ace_val)
    }
}

pub trait Deck {
    fn pick(&mut self) -> Option<Card>;
}

#[derive(Debug)]
pub struct DealerDeck {
    cards: Vec<Card>
}

impl DealerDeck {
    pub fn new() -> Self {
        let mut cards = vec![];
        for num_s in 1..5 {
            for num_v in 1..14 {
                let value = match num_v {
                    1 => Value::Ace,
                    2 => Value::Two,
                    3 => Value::Three,
                    4 => Value::Four,
                    5 => Value::Five,
                    6 => Value::Six,
                    7 => Value::Seven,
                    8 => Value::Eight,
                    9 => Value::Nine,
                    10 => Value::Ten,
                    11 => Value::Jack,
                    12 => Value::Queen,
                    13 => Value::King,
                    x => panic!("Card {} out of range!", x)
                };

                let suit = match num_s {
                    1 => Suit::Heart,
                    2 => Suit::Diamond,
                    3 => Suit::Spade,
                    4 => Suit::Club,
                    x => panic!("Suit {} out of range!", x)
                };
                cards.push(Card {value, suit});
            }
        }
        
        let mut rng = thread_rng();
        cards.shuffle(&mut rng);

        assert_eq!(cards.len(), 52);
        Self {cards}
    }
}

impl Deck for DealerDeck {
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

    pub fn sum(&self, card_count: usize, ace_val: i32) -> i32 {
        self.cards[..card_count].iter().map(|a| a.val(ace_val)).sum()
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
