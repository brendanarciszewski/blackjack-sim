pub mod attr;
use attr::*;

#[derive(Debug, PartialEq)]
pub struct Card {
    value: Value,
    suit: Suit
}

impl Card {
    pub fn new(value: Value, suit: Suit) -> Self {
        Self {value, suit}
    }

    pub fn value(&self) -> &Value {
        &self.value
    }

    pub fn suit(&self) -> &Suit {
        &self.suit
    }
}
