use std::cmp::Ordering;
use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Value {
    Numeral(u8),
    Jack,
    King,
    Queen,
    Ace,
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
    Value::Ace,
];

const ALL_SUITS: [Suit; 4] = [Suit::Clubs, Suit::Diamonds, Suit::Hearts, Suit::Spades];

#[derive(Copy, Clone, PartialEq, Eq)]
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

impl fmt::Debug for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Card {{ {} }}", self)
    }
}

impl std::str::FromStr for Card {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !s.is_ascii() {
            return Err("String must be ASCII");
        }

        if s.len() < 2 {
            return Err("String must contain value and suit");
        }

        let value_str = &s[0..s.len() - 1];

        let value = match value_str {
            "J" | "j" => Value::Jack,
            "Q" | "q" => Value::Queen,
            "K" | "k" => Value::King,
            "A" | "a" => Value::Ace,
            _ => {
                if let Ok(number) = value_str.parse::<u8>() {
                    if number >= 2 && number <= 10 {
                        Value::Numeral(number)
                    } else {
                        return Err("Numeric value out of range");
                    }
                } else {
                    return Err("Invalid value");
                }
            }
        };

        let suit_str = &s[s.len() - 1..s.len()];

        let suit = match suit_str {
            "C" | "c" => Suit::Clubs,
            "D" | "d" => Suit::Diamonds,
            "H" | "h" => Suit::Hearts,
            "S" | "s" => Suit::Spades,
            _ => {
                return Err("Invalid suit");
            }
        };

        Ok(Card::new(value, suit))
    }
}

impl Card {
    pub fn new(value: Value, suit: Suit) -> Self {
        Card { value, suit }
    }

    pub fn new_deck() -> Vec<Self> {
        let mut result: Vec<Self> = Vec::new();

        for suit in ALL_SUITS.iter() {
            for value in ALL_VALUES.iter() {
                result.push(Card::new(*value, *suit));
            }
        }

        result
    }

    pub fn try_vec_from<T: AsRef<str>>(value: T) -> Result<Vec<Self>, &'static str> {
        let mut result = vec![];
        let s = value.as_ref();

        for card_str in s.split_ascii_whitespace() {
            result.push(card_str.parse::<Card>()?);
        }

        Ok(result)
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
    use super::Card;
    use super::Suit::*;
    use super::Value::*;

    #[test]
    fn test_cmp_works() {
        assert!(Ace > Jack);
        assert!(Jack < Queen);
        assert!(Jack > Numeral(10));
    }

    #[test]
    fn test_display_works() {
        assert_eq!(
            format!("{}", Card::new(Numeral(12), Clubs)),
            String::from("12 of Clubs")
        )
    }

    #[test]
    fn test_parse_works() {
        assert_eq!(
            "10h".parse::<Card>().unwrap(),
            Card::new(Numeral(10), Hearts),
        );

        assert_eq!("kd".parse::<Card>().unwrap(), Card::new(King, Diamonds));
    }

    #[test]
    fn try_vec_from_works() {
        assert_eq!(
            Card::try_vec_from("2s qc").unwrap(),
            vec![Card::new(Numeral(2), Spades), Card::new(Queen, Clubs),]
        );
    }
}
