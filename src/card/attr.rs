//! Attributes of a Card
use std::fmt;

///The suit of the card
#[derive(Debug, PartialEq)]
pub enum Suit {
    Heart,
    Diamond,
    Spade,
    Club,
}

impl core::convert::TryFrom<u8> for Suit {
    type Error = &'static str;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Suit::Heart),
            1 => Ok(Suit::Diamond),
            2 => Ok(Suit::Spade),
            3 => Ok(Suit::Club),
            _ => Err("Outside Range"),
        }
    }
}

impl core::convert::Into<u8> for Suit {
    fn into(self) -> u8 {
        self as u8
    }
}

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Suit::*;
        write!(
            f,
            "{}",
            match self {
                Heart => '\u{2665}',
                Diamond => '\u{2666}',
                Spade => '\u{2660}',
                Club => '\u{2663}',
            }
        )
    }
}

///The value of the card
#[derive(Debug, PartialEq, Clone)]
pub enum Value {
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
    King,
}

impl Value {
    ///Get the Blackjack value of the card
    pub fn as_num(&self, ace_val: u8) -> u16 {
        match self {
            Value::Ace => ace_val as u16,
            Value::Jack | Value::Queen | Value::King => 10_u16,
            x => x.clone() as u16,
        }
    }
}

impl core::convert::TryFrom<u8> for Value {
    type Error = &'static str;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            1 => Ok(Value::Ace),
            2 => Ok(Value::Two),
            3 => Ok(Value::Three),
            4 => Ok(Value::Four),
            5 => Ok(Value::Five),
            6 => Ok(Value::Six),
            7 => Ok(Value::Seven),
            8 => Ok(Value::Eight),
            9 => Ok(Value::Nine),
            10 => Ok(Value::Ten),
            11 => Ok(Value::Jack),
            12 => Ok(Value::Queen),
            13 => Ok(Value::King),
            _ => Err("Outside Range"),
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Value::*;
        write!(
            f,
            "{}",
            match self {
                Ace => "A".into(),
                Jack => "J".into(),
                Queen => "Q".into(),
                King => "K".into(),
                v => format!("{}", v.as_num(1)),
            }
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::convert::{Into, TryFrom};

    #[test]
    fn num_2_10_get_value() {
        for i in 2..11 {
            let v = Value::try_from(i).unwrap();
            assert_eq!(v.as_num(1), i as u32);
            assert_eq!(v.as_num(11), i as u32);
        }
    }

    #[test]
    fn ace_get_value() {
        let v = Value::try_from(1).unwrap();
        assert_eq!(v.as_num(1), 1);
        assert_eq!(v.as_num(11), 11);
    }

    #[test]
    fn royalty_get_value() {
        for i in 11..14 {
            let v = Value::try_from(i).unwrap();
            assert_eq!(v.as_num(1), 10);
            assert_eq!(v.as_num(11), 10);
        }
    }

    #[test]
    fn get_suit() {
        for i in 0..4 {
            let s = Suit::try_from(i).unwrap();
            assert_eq!(Into::<u8>::into(s), i);
        }
    }
}
