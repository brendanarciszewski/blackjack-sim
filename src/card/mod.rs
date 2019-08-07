//! Useful model of a Card
pub mod attr;
use attr::*;
use std::fmt;

#[derive(Debug, PartialEq, Constructor)]
pub struct Card {
    value: Value,
    suit: Suit,
}

impl Card {
    pub fn value(&self) -> &Value {
        &self.value
    }

    pub fn suit(&self) -> &Suit {
        &self.suit
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        //TODO: format specifier currently not working
        write!(f, "{:_>2}{}", self.value, self.suit)
    }
}
