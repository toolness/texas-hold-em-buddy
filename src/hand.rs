use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt;

use super::card::{Card, Suit, Value};

#[derive(Debug, PartialOrd, PartialEq, Eq, Ord)]
/// This is ordered as per the listing here:
/// https://en.wikipedia.org/wiki/List_of_poker_hands
pub enum Category {
    HighCard(Value),
    OnePair(Value),
    TwoPair(Value, Value),
    ThreeOfAKind(Value),
    Straight(Value),
    Flush,
    FullHouse(Value, Value),
    FourOfAKind(Value),
    StraightFlush(Value),
}

#[derive(Debug, Eq, PartialEq)]
pub struct Hand {
    cards: Vec<Card>,
    grouped_by_n_of_a_kind: Vec<(usize, Value, Vec<Card>)>,
    grouped_by_values: Vec<(Value, Vec<Card>)>,
    grouped_by_suits: Vec<(usize, Suit, Vec<Card>)>,
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
                let best = self.find_best_category().expect("Hand is non-empty");
                let other_best = other.find_best_category().expect("Hand is non-empty");

                let best_cmp = best.cmp(&other_best);

                if best_cmp != Ordering::Equal {
                    return best_cmp;
                }

                // It looks like the two hands are tied, so we'll try to break the
                // tie by seeing if one has a higher card.
                for (card, other_card) in self.cards.iter().rev().zip(other.cards.iter().rev()) {
                    let cmp = card.cmp(other_card);
                    if cmp != Ordering::Equal {
                        return cmp;
                    }
                }
                Ordering::Equal
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

        let mut grouped_by_suits = suits
            .into_iter()
            .map(|(suit, cards)| (cards.len(), suit, cards))
            .collect::<Vec<_>>();
        grouped_by_suits.sort_unstable_by(|(a, _, _), (b, _, _)| a.cmp(&b));

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
            grouped_by_suits,
        }
    }
}

impl Hand {
    pub fn highest_value(&self) -> Option<Value> {
        match self.grouped_by_values.last() {
            Some((v, _)) => Some(*v),
            None => None,
        }
    }

    pub fn four_of_a_kind(&self) -> Option<Value> {
        match self.grouped_by_n_of_a_kind.as_slice() {
            [.., (4, value, _)] => Some(*value),
            _ => None,
        }
    }

    pub fn three_of_a_kind(&self) -> Option<Value> {
        match self.grouped_by_n_of_a_kind.as_slice() {
            [.., (3, value, _)] => Some(*value),
            _ => None,
        }
    }

    pub fn one_pair(&self) -> Option<Value> {
        match self.grouped_by_n_of_a_kind.as_slice() {
            [.., (2, value, _)] => Some(*value),
            _ => None,
        }
    }

    pub fn full_house(&self) -> Option<(Value, Value)> {
        match self.grouped_by_n_of_a_kind.as_slice() {
            [.., (2, pair_value, _), (3, triplet_value, _)] => Some((*triplet_value, *pair_value)),
            // On a 7-card Texas Hold-Em "hand", we might actually have *two* triplets, which
            // we should "downgrade" to a full house.
            [.., (3, lower_triplet, _), (3, higher_triplet, _)] => {
                Some((*higher_triplet, *lower_triplet))
            }
            _ => None,
        }
    }

    pub fn two_pair(&self) -> Option<(Value, Value)> {
        match self.grouped_by_n_of_a_kind.as_slice() {
            [.., (2, lower_pair, _), (2, higher_pair, _)] => Some((*higher_pair, *lower_pair)),
            _ => None,
        }
    }

    pub fn flush(&self) -> Option<(Suit, Vec<Card>)> {
        match self.grouped_by_suits.as_slice() {
            [.., (n, suit, cards)] => {
                if *n >= 5 {
                    Some((*suit, cards.clone()))
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    fn get_straight(&self, cards: &Vec<(Value, Vec<Card>)>) -> Option<Value> {
        let mut latest: Option<(Value, u8, usize)> = None;
        let mut has_ace = false;

        for (value, _) in cards.iter().rev() {
            if *value == Value::Ace {
                has_ace = true;
            }
            let num_value = u8::from(value);
            if let Some((high_value, prev_num_value, run_len)) = latest {
                if num_value + 1 == prev_num_value {
                    let new_run_len = run_len + 1;
                    let is_straight_with_ace_low =
                        new_run_len == 4 && *value == Value::Two && has_ace;
                    if new_run_len == 5 || is_straight_with_ace_low {
                        return Some(high_value);
                    }
                    latest = Some((high_value, num_value, new_run_len));
                    continue;
                }
            }
            latest = Some((*value, num_value, 1));
        }

        None
    }

    pub fn straight_flush(&self) -> Option<Value> {
        if let Some((_, cards)) = self.flush() {
            self.get_straight(
                &cards
                    .into_iter()
                    .map(|card| (card.value, vec![card]))
                    .collect::<Vec<_>>(),
            )
        } else {
            None
        }
    }

    pub fn straight(&self) -> Option<Value> {
        self.get_straight(&self.grouped_by_values)
    }

    pub fn find_best_category(&self) -> Option<Category> {
        if let Some(value) = self.straight_flush() {
            Some(Category::StraightFlush(value))
        } else if let Some(value) = self.four_of_a_kind() {
            Some(Category::FourOfAKind(value))
        } else if let Some((triplet_value, pair_value)) = self.full_house() {
            Some(Category::FullHouse(triplet_value, pair_value))
        } else if self.flush().is_some() {
            Some(Category::Flush)
        } else if let Some(value) = self.straight() {
            Some(Category::Straight(value))
        } else if let Some(value) = self.three_of_a_kind() {
            Some(Category::ThreeOfAKind(value))
        } else if let Some((higher_pair, lower_pair)) = self.two_pair() {
            Some(Category::TwoPair(higher_pair, lower_pair))
        } else if let Some(value) = self.one_pair() {
            Some(Category::OnePair(value))
        } else if let Some(value) = self.highest_value() {
            Some(Category::HighCard(value))
        } else {
            None
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
    use std::cmp::Ordering;

    use super::super::card::{Suit, Value};
    use super::{Category, Hand};

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
    fn test_full_house_works() {
        assert_eq!(hand("").full_house(), None);
        assert_eq!(
            hand("2s 2d 2h 3h 3d").full_house(),
            Some((Value::Two, Value::Three))
        );
    }

    #[test]
    fn test_full_house_handles_double_triplets() {
        assert_eq!(
            hand("2s 2d 2h 3h 3d 3c").full_house(),
            Some((Value::Three, Value::Two))
        );
    }

    #[test]
    fn test_two_pair_works() {
        assert_eq!(hand("").two_pair(), None);
        assert_eq!(
            hand("2s 2d 3h 3d").two_pair(),
            Some((Value::Three, Value::Two))
        );
    }

    #[test]
    fn test_flush_works() {
        assert_eq!(hand("").flush(), None);
        assert_eq!(hand("kh 3h 2h 7h 5h").flush().unwrap().0, Suit::Hearts);
        assert_eq!(hand("kh 3h 2h 7h 5h 9h").flush().unwrap().0, Suit::Hearts);
        assert_eq!(
            hand("kh 3h 2h 7h 5h 9h 10h").flush().unwrap().0,
            Suit::Hearts
        );
        assert_eq!(hand("2s 2d 2h 2c 5h").flush(), None);
    }

    #[test]
    fn test_straight_flush_works() {
        assert_eq!(hand("").straight_flush(), None);
        assert_eq!(hand("kh 10h jh qh ah").straight_flush(), Some(Value::Ace));
    }

    #[test]
    fn test_straight_works() {
        assert_eq!(hand("").straight(), None);
        assert_eq!(hand("2s 3h 4d 5h 6s").straight(), Some(Value::Six));
        assert_eq!(hand("2s 3h 4d 5h 6s 7d 8h").straight(), Some(Value::Eight));
    }

    #[test]
    fn test_straight_works_with_ace_high() {
        assert_eq!(hand("10s jd qc kh as").straight(), Some(Value::Ace));
    }

    #[test]
    fn test_straight_works_with_ace_low() {
        assert_eq!(hand("as 2s 3h 4d 5h").straight(), Some(Value::Five));
    }

    #[test]
    fn test_ord_considers_kickers() {
        // https://en.wikipedia.org/wiki/Kicker_(poker)
        assert!(hand("qs qc 10s 5s 3s") > hand("qh qd 10h 4h 3s"));
        assert!(hand("qs qc 10s 5s 3s") < hand("qh qd kh 4h 3s"));
    }

    #[test]
    fn test_ord_works_for_empty_hands() {
        assert!(hand("as") > hand(""));
        assert!(hand("") < hand("kh"));
        assert!(hand("").cmp(&hand("")) == Ordering::Equal);
    }

    #[test]
    fn test_ord_works_for_high_cards() {
        assert!(hand("2h as") > hand("kd qs"));
        assert!(hand("as kd") > hand("qd as"));
        assert!(hand("as").cmp(&hand("ah")) == Ordering::Equal);
    }

    #[test]
    fn test_ord_works_for_two_pairs() {
        assert!(hand("3h 3s") > hand("2h 2s"));
        assert!(hand("3h 3s") > hand("kh qd"));
        assert!(hand("3h 3s").cmp(&hand("3d 3c")) == Ordering::Equal);
    }

    #[test]
    fn test_category_enum_orders_as_expected() {
        assert!(Category::StraightFlush(Value::Two) > Category::FourOfAKind(Value::Two));
        assert!(
            Category::FullHouse(Value::Three, Value::Four)
                > Category::FullHouse(Value::Three, Value::Two)
        );
    }

    #[test]
    fn test_find_best_category_works() {
        assert_eq!(hand("").find_best_category(), None);

        assert_eq!(
            hand("2h kd").find_best_category(),
            Some(Category::HighCard(Value::King))
        );

        assert_eq!(
            hand("2h 2d").find_best_category(),
            Some(Category::OnePair(Value::Two))
        );

        assert_eq!(
            hand("2h 3h 2d 3c").find_best_category(),
            Some(Category::TwoPair(Value::Three, Value::Two))
        );

        assert_eq!(
            hand("2h 3h 4s 5c 6h").find_best_category(),
            Some(Category::Straight(Value::Six))
        );

        assert_eq!(
            hand("2h 3h 4h kh 10h").find_best_category(),
            Some(Category::Flush)
        );

        assert_eq!(
            hand("2h 3h 2d 3c 3s").find_best_category(),
            Some(Category::FullHouse(Value::Three, Value::Two))
        );

        assert_eq!(
            hand("5s 5h 5c 5d").find_best_category(),
            Some(Category::FourOfAKind(Value::Five))
        );

        assert_eq!(
            hand("2h 3h 4h 5h 6h").find_best_category(),
            Some(Category::StraightFlush(Value::Six))
        );
    }
}
