use super::card::Card;

#[derive(Debug)]
pub struct Hand {
    cards: Vec<Card>
}

impl From<Vec<Card>> for Hand {
    fn from(mut cards: Vec<Card>) -> Hand {
        cards.sort_unstable();
        Hand { cards }
    }
}
