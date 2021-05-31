mod random;
mod card;
mod hand;

fn main() {
    use card::Card;
    use hand::Hand;

    let mut r = random::Random { seed: 15 };
    let deck = Card::new_deck();
    let hand = "kd ac 2s".parse::<Hand>().unwrap();
    println!("Here's a random number: {}", r.next_float());
    println!("Here's a deck with {} cards: {:?}", deck.len(), deck);
    println!("Here's a hand: {:?}", hand);
}
