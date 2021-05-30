mod random;
mod card;
mod hand;

fn main() {
    use card::{Card, Value, Suit};
    let mut r = random::Random { seed: 15 };
    let deck = Card::new_deck();
    let hand = hand::Hand::from(vec![
        Card { value: Value::Ace, suit: Suit::Clubs },
    ]);
    println!("Here's a random number: {}", r.next_float());
    println!("Here's a deck with {} cards: {:?}", deck.len(), deck);
    println!("Here's a hand: {:?}", hand);
}
