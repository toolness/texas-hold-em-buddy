use std::collections::HashMap;
use std::fmt;

use super::card::{Card, Suit, Value};

#[derive(Debug)]
pub struct Hand {
    cards: Vec<Card>,
    grouped_by_values: Vec<(Value, Vec<Card>)>,
    grouped_by_suits: Vec<(Suit, Vec<Card>)>,
}

fn append_to<K: std::hash::Hash + Eq>(key: K, value: Card, map: &mut HashMap<K, Vec<Card>>) {
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
            append_to(card.value, *card, &mut values);
            append_to(card.suit, *card, &mut suits);
        }

        let mut grouped_by_values = values.into_iter().collect::<Vec<_>>();
        grouped_by_values.sort_by(|(a, _), (b, _)| a.cmp(b));

        Hand {
            cards,
            grouped_by_values,
            grouped_by_suits: suits.into_iter().collect::<Vec<_>>(),
        }
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
