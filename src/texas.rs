use std::collections::HashMap;
use std::ops::Deref;

use super::card::Card;
use super::hand::{Category, Hand};
use super::random::Random;

struct Counters(HashMap<&'static str, usize>);

impl Deref for Counters {
    type Target = HashMap<&'static str, usize>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Counters {
    pub fn increment(&mut self, key: &'static str) {
        let counter = self.0.entry(key).or_insert(0);
        *counter += 1;
    }

    pub fn print_percentages(&self) {
        let mut entries = self.0.clone().into_iter().collect::<Vec<_>>();
        let total = entries.iter().fold(0, |acc, (_, value)| acc + value) as f64;

        entries.sort();

        for (key, count) in entries {
            let pct = count as f64 / total * 100.0;
            println!("  {:20} {:.1}%", key, pct);
        }
    }
}

fn category_to_str(cat: &Category) -> &'static str {
    match cat {
        Category::HighCard(_) => "High card",
        Category::OnePair(_) => "One pair",
        Category::TwoPair(_, _) => "Two pair",
        Category::ThreeOfAKind(_) => "Three of a kind",
        Category::Straight(_) => "Straight",
        Category::Flush(_, _, _, _, _) => "Flush",
        Category::FullHouse(_, _) => "Full house",
        Category::FourOfAKind(_) => "Four of a kind",
        Category::StraightFlush(_) => "Straight flush",
    }
}

fn remove_from_deck(mut deck: Vec<Card>, cards: Vec<Card>) -> Vec<Card> {
    deck.retain(|card| !cards.contains(&card));
    deck
}

const NUM_COMMUNITY_CARDS: usize = 5;
const NUM_HOLE_CARDS: usize = 2;

pub fn run_texas_hold_em(
    hole_cards: Vec<Card>,
    src_community_cards: Vec<Card>,
    num_iterations: usize,
    mut random: Random,
) {
    let mut outcomes = Counters(HashMap::new());
    let orig_deck = remove_from_deck(
        Card::new_deck(),
        [src_community_cards.clone(), hole_cards.clone()].concat(),
    );

    println!(
        "Hole cards:\n  {}\nCommunity cards:\n  {}\n",
        Card::vec_to_string(&hole_cards),
        Card::vec_to_string(&src_community_cards)
    );

    assert_eq!(hole_cards.len(), NUM_HOLE_CARDS, "Must have 2 hole cards");

    if src_community_cards.len() >= NUM_COMMUNITY_CARDS {
        println!("No community cards need to be drawn. Nothing to do!");
        return;
    }

    let num_cards_to_draw = NUM_COMMUNITY_CARDS - src_community_cards.len();

    for _ in 0..num_iterations {
        let mut deck = orig_deck.clone();
        let mut community_cards = src_community_cards.clone();
        random.shuffle(&mut deck);

        for _ in 0..num_cards_to_draw {
            community_cards.push(deck.pop().unwrap());
        }

        let hand = Hand::from([community_cards, hole_cards.clone()].concat());
        let cat = hand.find_best_category().unwrap();
        outcomes.increment(category_to_str(&cat));
    }

    println!(
        "Outcome distribution after randomly drawing {} more community cards {} times:\n",
        num_cards_to_draw, num_iterations
    );

    outcomes.print_percentages();
}
