use std::cmp::Ordering;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Value {
    Numeral(u8),
    Jack,
    King,
    Queen,
    Ace
}

#[derive(Debug, Copy, Clone)]
pub enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

const ALL_VALUES: [Value; 13] = [
    Value::Numeral(2),
    Value::Numeral(3),
    Value::Numeral(4),
    Value::Numeral(5),
    Value::Numeral(6),
    Value::Numeral(7),
    Value::Numeral(8),
    Value::Numeral(9),
    Value::Numeral(10),
    Value::Jack,
    Value::King,
    Value::Queen,
    Value::Ace
];

const ALL_SUITS: [Suit; 4] = [
    Suit::Clubs,
    Suit::Diamonds,
    Suit::Hearts,
    Suit::Spades,
];

#[derive(Debug)]
pub struct Card {
    value: Value,
    suit: Suit,
}

impl Card {
    pub fn new_deck() -> Vec<Self> {
        let mut result: Vec<Self> = Vec::new();

        for suit in ALL_SUITS.iter() {
            for value in ALL_VALUES.iter() {
                result.push(Card { suit: *suit, value: *value });
            }
        }

        result
    }
}

impl From<&Value> for u8 {
    fn from(value: &Value) -> u8 {
        match value {
            Value::Numeral(x) => *x,
            Value::Jack => 11,
            Value::Queen => 12,
            Value::King => 13,
            // Always treat Ace as high.
            Value::Ace => 14,
        }
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Value) -> Option<Ordering> {
        let my_value = u8::from(self);
        let other_value = u8::from(other);

        Some(my_value.cmp(&other_value))
    }
}

#[cfg(Test)]
mod tests {
    fn test_cmp_works() {
        assert!(Value::Ace > Value::Jack);
        assert!(Value::Jack < Value::Queen);
        assert!(Value::Jack > Value::Numeral(10));
    }
}
