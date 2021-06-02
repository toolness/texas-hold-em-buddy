#[macro_use(value_t)]
extern crate clap;

use clap::{App, Arg, SubCommand};

mod card;
mod hand;
mod random;
mod texas;

const AFTER_HELP: &'static str = "\
EXAMPLES:
    theb besthand \"qs 2s 3d jh kc\"
    theb play \"10s js\"
    theb play \"10s js\" \"qs 9s 3d\"
";

fn main() {
    use card::Card;
    use hand::Hand;
    use random::Random;

    let matches = App::new("Texas Hold 'Em Buddy")
        .version("1.0")
        .author("Atul Varma <varmaa@gmail.com>")
        .about("An assistant for analyzing Texas Hold 'Em games")
        .after_help(AFTER_HELP)
        .subcommand(
            SubCommand::with_name("besthand")
                .about("Attempts to deduce the best hand from a list of cards")
                .arg(
                    Arg::with_name("CARDS")
                        .help("List of cards")
                        .required(true)
                        .index(1),
                ),
        )
        .subcommand(
            SubCommand::with_name("play")
                .about(
                    "Attempts to simulate play with the given cards and reports probable outcomes",
                )
                .arg(
                    Arg::with_name("HOLE_CARDS")
                        .help("List of two hole cards")
                        .required(true)
                        .index(1),
                )
                .arg(
                    Arg::with_name("COMMUNITY_CARDS")
                        .help("List of up to five community cards")
                        .required(false)
                        .index(2),
                )
                .arg(
                    Arg::with_name("times")
                        .short("t")
                        .long("times")
                        .value_name("N")
                        .default_value("100000")
                        .help("Number of times to simulate play")
                        .takes_value(true),
                ),
        )
        .subcommand(SubCommand::with_name("test").about("Runs a manual test"))
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("besthand") {
        let hand = matches
            .value_of("CARDS")
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
    } else if let Some(matches) = matches.subcommand_matches("play") {
        let hole_cards = Card::try_vec_from(matches.value_of("HOLE_CARDS").unwrap()).unwrap();
        let community_cards =
            Card::try_vec_from(matches.value_of("COMMUNITY_CARDS").unwrap_or_default()).unwrap();
        let times = value_t!(matches.value_of("times"), usize).unwrap_or_else(|e| e.exit());

        texas::run_texas_hold_em(hole_cards, community_cards, times, Random::new());
    } else if let Some(_) = matches.subcommand_matches("test") {
        let mut r = Random::new();
        let mut deck = Card::new_deck();
        r.shuffle(&mut deck);
        let hand = Hand::from(deck[0..7].to_owned());
        println!("Here's a hand:\n  {}", hand);
        println!("Its best category is:\n  {:?}", hand.find_best_category());
    }
}
