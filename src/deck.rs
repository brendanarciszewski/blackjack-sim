use rand::prelude::*;
use std::ops::Index;
use std::marker::Sized;

trait IntegerToEnum {
    fn get_item(val: i32) -> Option<Self> where Self: Sized;
}

#[derive(Debug, PartialEq)]
enum Suit {
    Heart,
    Diamond,
    Spade,
    Club
}

impl IntegerToEnum for Suit {
    fn get_item(val: i32) -> Option<Self> {
        match val {
            0 => Some(Suit::Heart),
            1 => Some(Suit::Diamond),
            2 => Some(Suit::Spade),
            3 => Some(Suit::Club),
            _ => None
        }
    }
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

impl IntegerToEnum for Value {
    fn get_item(val: i32) -> Option<Self> {
        match val {
            1 => Some(Value::Ace),
            2 => Some(Value::Two),
            3 => Some(Value::Three),
            4 => Some(Value::Four),
            5 => Some(Value::Five),
            6 => Some(Value::Six),
            7 => Some(Value::Seven),
            8 => Some(Value::Eight),
            9 => Some(Value::Nine),
            10 => Some(Value::Ten),
            11 => Some(Value::Jack),
            12 => Some(Value::Queen),
            13 => Some(Value::King),
            _ => None
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
        for num_s in 0..4 {
            for num_v in 1..14 {
                let value = Value::get_item(num_v).unwrap();
                let suit = Suit::get_item(num_s).unwrap();
                cards.push(Card {value, suit});
            }
        }
        
        let mut rng = thread_rng();
        cards.shuffle(&mut rng);

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
        let d = DealerDeck::new();
        assert_eq!(d.cards.len(), 52);
    }

    #[test]
    fn card_2_10_get_value() {
        for i in 2..11 {
            let v = Value::get_item(i).unwrap();
            assert_eq!(v.val(1), i);
            assert_eq!(v.val(11), i);
        }
    }

    #[test]
    fn card_ace_get_value() {
        let v = Value::get_item(1).unwrap();
        assert_eq!(v.val(1), 1);
        assert_eq!(v.val(11), 11);
    }

    #[test]
    fn card_royalty_get_value() {
        for i in 11..14 {
            let v = Value::get_item(i).unwrap();
            assert_eq!(v.val(1), 10);
            assert_eq!(v.val(11), 10);
        }
    }

    #[test]
    fn card_get_suit() {
        for i in 0..4 {
            let s = Suit::get_item(i).unwrap();
            assert_eq!(s as i32, i);
        }
    }

}