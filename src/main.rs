use serde::Deserialize;

mod card;
mod hand;
mod random;
mod texas;

const VERSION: &'static str = "1.0.0";

const USAGE: &'static str = "
Perform various poker-related tasks.

Usage:
  poker-fun test
  poker-fun besthand <hand>
  poker-fun texas <hand> [--community=<hand>] [--times=<times>]
  poker-fun --version
  poker-fun (-h | --help)

Options:
  -h --help     Show this screen.
  --version     Show version.

Examples:
  poker-fun besthand \"qs 2s 3d jh kc\"
  poker-fun texas \"10s js\"
  poker-fun texas \"10s js\" --community=\"qs 9s 3d\"
";

#[derive(Debug, Deserialize)]
struct Args {
    cmd_besthand: bool,
    cmd_test: bool,
    cmd_texas: bool,
    flag_community: Option<String>,
    flag_times: Option<usize>,
    arg_hand: Option<String>,
}

fn main() {
    use card::Card;
    use hand::Hand;
    use random::Random;

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
    } else if args.cmd_texas {
        let hole_cards = Card::try_vec_from(args.arg_hand.unwrap()).unwrap();
        let community_cards = Card::try_vec_from(args.flag_community.unwrap_or_default()).unwrap();
        let times = args.flag_times.unwrap_or(100_000);

        texas::run_texas_hold_em(hole_cards, community_cards, times, Random::new());
    } else if args.cmd_test {
        let mut r = Random::new();
        let mut deck = Card::new_deck();
        r.shuffle(&mut deck);
        let hand = Hand::from(deck[0..7].to_owned());
        println!("Here's a hand:\n  {}", hand);
        println!("Its best category is:\n  {:?}", hand.find_best_category());
    }
}
