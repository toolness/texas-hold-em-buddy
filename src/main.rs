mod random;
mod card;

fn main() {
    let mut r = random::Random { seed: 15 };
    let deck = card::Card::new_deck();
    println!("Here's a random number: {}", r.next_float());
    println!("Here's a deck with {} cards: {:?}", deck.len(), deck);
}
