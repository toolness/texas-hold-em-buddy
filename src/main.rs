mod random;
mod card;
mod hand;

fn main() {
    use card::Card;
    let mut r = random::Random { seed: 15 };
    let deck = Card::new_deck();
    let hand = hand::Hand::from(Card::try_vec_from("kd ac 2s").unwrap());
    println!("Here's a random number: {}", r.next_float());
    println!("Here's a deck with {} cards: {:?}", deck.len(), deck);
    println!("Here's a hand: {:?}", hand);
}
