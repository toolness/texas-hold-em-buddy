use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt;

use super::card::{Card, Suit, Value};

#[derive(Debug, Eq, PartialEq)]
pub struct Hand {
    cards: Vec<Card>,
    grouped_by_n_of_a_kind: Vec<(usize, Value, Vec<Card>)>,
    grouped_by_values: Vec<(Value, Vec<Card>)>,
    grouped_by_suits: Vec<(Suit, Vec<Card>)>,
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Hand) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Hand) -> Ordering {
        if self.is_empty() {
            if other.is_empty() {
                Ordering::Equal
            } else {
                Ordering::Less
            }
        } else {
            if other.is_empty() {
                Ordering::Greater
            } else {
                // TODO: Actually compare hands as per the ranking
                // outlined at https://en.wikipedia.org/wiki/List_of_poker_hands.
                self.highest_value()
                    .unwrap()
                    .cmp(&other.highest_value().unwrap())
            }
        }
    }
}

fn append_to_hashmap_vec<K: std::hash::Hash + Eq, V>(
    key: K,
    value: V,
    map: &mut HashMap<K, Vec<V>>,
) {
    if let Some(arr) = map.get_mut(&key) {
        arr.push(value);
    } else {
        map.insert(key, vec![value]);
    }
}

impl From<Vec<Card>> for Hand {
    fn from(mut cards: Vec<Card>) -> Hand {
        cards.sort_unstable();
        let mut values: HashMap<Value, Vec<Card>> = HashMap::new();
        let mut suits: HashMap<Suit, Vec<Card>> = HashMap::new();

        for card in cards.iter() {
            append_to_hashmap_vec(card.value, *card, &mut values);
            append_to_hashmap_vec(card.suit, *card, &mut suits);
        }

        let mut grouped_by_values = values.into_iter().collect::<Vec<_>>();
        grouped_by_values.sort_unstable_by(|(a, _), (b, _)| a.cmp(b));

        let mut grouped_by_n_of_a_kind = grouped_by_values
            .iter()
            .map(|(value, cards)| (cards.len(), *value, cards.clone()))
            .collect::<Vec<_>>();
        grouped_by_n_of_a_kind.sort_unstable_by(|(a_len, a_val, _), (b_len, b_val, _)| {
            let len_cmp = a_len.cmp(b_len);

            if len_cmp == Ordering::Equal {
                a_val.cmp(b_val)
            } else {
                len_cmp
            }
        });

        Hand {
            cards,
            grouped_by_values,
            grouped_by_n_of_a_kind,
            grouped_by_suits: suits.into_iter().collect::<Vec<_>>(),
        }
    }
}

impl Hand {
    pub fn highest_value(&self) -> Option<Value> {
        if let Some((v, _)) = self.grouped_by_values.last() {
            Some(*v)
        } else {
            None
        }
    }

    pub fn four_of_a_kind(&self) -> Option<Value> {
        match self.grouped_by_n_of_a_kind.as_slice() {
            [.., (4, value, _)] => Some(*value),
            _ => None,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.cards.is_empty()
    }
}

impl Into<Vec<Card>> for Hand {
    fn into(self) -> Vec<Card> {
        self.cards
    }
}

impl std::str::FromStr for Hand {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Hand::from(Card::try_vec_from(s)?))
    }
}

impl fmt::Display for Hand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let cards_vec: Vec<String> = self.cards.iter().map(|card| format!("{}", card)).collect();

        write!(f, "{}", cards_vec.join(", "))
    }
}

#[cfg(test)]
mod tests {
    use super::super::card::Value;
    use super::Hand;

    fn hand(value: &'static str) -> Hand {
        value.parse::<Hand>().unwrap()
    }

    #[test]
    fn test_highest_value_works() {
        assert_eq!(hand("").highest_value(), None);
        assert_eq!(hand("2s ah 4d").highest_value(), Some(Value::Ace));
    }

    #[test]
    fn test_four_of_a_kind_works() {
        assert_eq!(hand("").four_of_a_kind(), None);
        assert_eq!(hand("2s").four_of_a_kind(), None);
        assert_eq!(hand("2s 2d 2h 2c").four_of_a_kind(), Some(Value::Two));
        assert_eq!(
            hand("kh 3s 3d qs 3h 4d 3c").four_of_a_kind(),
            Some(Value::Three)
        );
    }

    #[test]
    fn test_ord_works_for_empty_hands() {
        assert!(hand("as") > hand(""));
        assert!(hand("") < hand("kh"));
        assert!(hand("").cmp(&hand("")) == std::cmp::Ordering::Equal);
    }

    #[test]
    fn test_ord_works_for_high_cards() {
        assert!(hand("2h as") > hand("kd qs"));
    }
}
