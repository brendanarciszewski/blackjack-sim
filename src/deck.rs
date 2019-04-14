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
    fn val(&self) -> i32 {
        match self {
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
    pub fn val(&self) -> i32 {
        self.value.val()
    }
}

#[derive(Debug)]
pub struct Deck {
    cards: Vec<Card>
}

impl Deck {
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
        
        use rand::prelude::*;
        let mut rng = thread_rng();
        cards.shuffle(&mut rng);

        assert_eq!(cards.len(), 52);
        Deck {cards}
    }

    pub fn pick(&mut self) -> Option<Card> {
        self.cards.pop()
    }
}