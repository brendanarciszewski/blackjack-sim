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

    pub fn val(&self, ace_val: i32) -> i32 {
        self.value.val(ace_val)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
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