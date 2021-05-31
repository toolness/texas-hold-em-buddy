use std::cmp::Ordering;
use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Value {
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
    King,
    Queen,
    Ace,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

const ALL_VALUES: [Value; 13] = [
    Value::Two,
    Value::Three,
    Value::Four,
    Value::Five,
    Value::Six,
    Value::Seven,
    Value::Eight,
    Value::Nine,
    Value::Ten,
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
        write!(f, "{:?}", self)
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
            "2" => Value::Two,
            "3" => Value::Three,
            "4" => Value::Four,
            "5" => Value::Five,
            "6" => Value::Six,
            "7" => Value::Seven,
            "8" => Value::Eight,
            "9" => Value::Nine,
            "10" => Value::Ten,
            _ => {
                return Err("Invalid value");
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
            Value::Two => 2,
            Value::Three => 3,
            Value::Four => 4,
            Value::Five => 5,
            Value::Six => 6,
            Value::Seven => 7,
            Value::Eight => 8,
            Value::Nine => 9,
            Value::Ten => 10,
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
        assert!(Jack > Ten);
    }

    #[test]
    fn test_display_works() {
        assert_eq!(
            format!("{}", Card::new(Seven, Clubs)),
            String::from("Seven of Clubs")
        )
    }

    #[test]
    fn test_parse_works() {
        assert_eq!("10h".parse::<Card>().unwrap(), Card::new(Ten, Hearts),);

        assert_eq!("kd".parse::<Card>().unwrap(), Card::new(King, Diamonds));
    }

    #[test]
    fn try_vec_from_works() {
        assert_eq!(
            Card::try_vec_from("2s qc").unwrap(),
            vec![Card::new(Two, Spades), Card::new(Queen, Clubs),]
        );
    }
}
