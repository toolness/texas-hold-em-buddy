use std::cmp::Ordering;
use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Value {
    Numeral(u8),
    Jack,
    King,
    Queen,
    Ace
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
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

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Card {
    pub value: Value,
    pub suit: Suit,
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Value::Numeral(x) = self {
            write!(f, "{}", x)
        } else {
            write!(f, "{:?}", self)
        }
    }
}

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} of {}", self.value, self.suit)
    }
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
        Some(self.cmp(&other))
    }
}

impl Ord for Value {
    fn cmp(&self, other: &Value) -> Ordering {
        let my_value = u8::from(self);
        let other_value = u8::from(other);

        my_value.cmp(&other_value)
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Card) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Card) -> Ordering {
        // Note that we're only comparing the card's value and
        // ignoring the suit.
        self.value.cmp(&other.value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cmp_works() {
        assert!(Value::Ace > Value::Jack);
        assert!(Value::Jack < Value::Queen);
        assert!(Value::Jack > Value::Numeral(10));
    }

    #[test]
    fn test_display_works() {
        assert_eq!(
            format!("{}", Card { suit: Suit::Clubs, value: Value::Numeral(12) }),
            String::from("12 of Clubs")
        )
    }
}
