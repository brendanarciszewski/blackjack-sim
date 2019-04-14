use std::marker::Sized;

pub trait IntegerToEnum {
    fn get_item(val: i32) -> Option<Self> where Self: Sized;
}

#[derive(Debug, PartialEq)]
pub enum Suit {
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
    King
}

impl Value {
    pub fn val(&self, ace_val: i32) -> i32 {
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
