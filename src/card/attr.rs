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
    pub fn as_num(&self, ace_val: u32) -> u32 {
        match self {
            Value::Ace => ace_val,
            Value::Jack | Value::Queen | Value::King => 10_u32,
            x => x.clone() as u32 
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

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn num_2_10_get_value() {
        for i in 2..11 {
            let v = Value::get_item(i).unwrap();
            assert_eq!(v.as_num(1), i as u32);
            assert_eq!(v.as_num(11), i as u32);
        }
    }

    #[test]
    fn ace_get_value() {
        let v = Value::get_item(1).unwrap();
        assert_eq!(v.as_num(1), 1);
        assert_eq!(v.as_num(11), 11);
    }

    #[test]
    fn royalty_get_value() {
        for i in 11..14 {
            let v = Value::get_item(i).unwrap();
            assert_eq!(v.as_num(1), 10);
            assert_eq!(v.as_num(11), 10);
        }
    }

    #[test]
    fn get_suit() {
        for i in 0..4 {
            let s = Suit::get_item(i).unwrap();
            assert_eq!(s as i32, i);
        }
    }
}