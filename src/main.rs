use serde::Deserialize;

mod card;
mod hand;
mod random;

const VERSION: &'static str = "1.0.0";

const USAGE: &'static str = "
Perform various poker-related tasks.

Usage:
  poker-fun test
  poker-fun besthand <hand>
  poker-fun --version
  poker-fun (-h | --help)

Options:
  -h --help     Show this screen.
  --version     Show version.

Examples:
  poker-fun besthand \"qs 2s 3d jh kc\"
";

#[derive(Debug, Deserialize)]
struct Args {
    cmd_besthand: bool,
    cmd_test: bool,
    arg_hand: Option<String>,
}

fn main() {
    use chrono::prelude::*;

    use card::Card;
    use hand::Hand;

    let version = VERSION.to_owned();
    let args: Args = docopt::Docopt::new(USAGE)
        .and_then(|d| d.version(Some(version)).deserialize())
        .unwrap_or_else(|e| e.exit());

    if args.cmd_besthand {
        let hand = args
            .arg_hand
            .unwrap()
            .parse::<Hand>()
            .expect("Hand argument should be valid");
        let opt_cat = hand.find_best_category();
        if let Some(cat) = opt_cat {
            let kickers = cat.get_kickers(&hand);
            println!("The best hand for\n  {}\nis\n  {:?}", hand, cat);
            if kickers.is_empty() {
                println!("with no kickers.");
            } else {
                println!("with kickers\n  {}.", Card::vec_to_string(&kickers));
            }
        } else {
            println!("The hand you provided is empty.");
        }
    } else if args.cmd_test {
        let mut r = random::Random {
            seed: Utc::now().timestamp() as u64,
        };
        let mut deck = Card::new_deck();
        r.shuffle(&mut deck);
        let hand = Hand::from(deck[0..7].to_owned());
        println!("Here's a hand:\n  {}", hand);
        println!("Its best category is:\n  {:?}", hand.find_best_category());
    }
}
