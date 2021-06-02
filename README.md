This is a simple tool to perform analysis of [Texas Hold 'Em][the] hands.

[the]: https://en.wikipedia.org/wiki/Texas_hold_'em

## Motivation

I was reading Maria Konnikova's [The Biggest Bluff][mk] and wanted an excuse to
write Rust, as well as obtain a better understanding of how Texas Hold 'Em works.

[mk]: https://www.mariakonnikova.com/books/the-biggest-bluff/

## Description

Here's the command-line help:

```
Texas Hold 'Em Buddy 1.0
Atul Varma <varmaa@gmail.com>
An assistant for analyzing Texas Hold 'Em games

USAGE:
    theb [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    besthand    Attempts to deduce the best hand from a list of cards
    help        Prints this message or the help of the given subcommand(s)
    play        Attempts to simulate play with the given cards and reports probable outcomes
    test        Runs a manual test

EXAMPLES:
    Find the best possible hand for a Two of Spades, Three of Diamonds, Jack of Hearts,
    Queen of Spades, and King of Clubs:

        theb besthand qs 2s 3d jh kc

    Simulate play and report outcomes given hole cards Ten and Jack of Spades:

        theb play 10s js

    Do the same as above, but with community cards Queen and Nine of Spades and
    Three of Diamonds in the mix:

        theb play 10s js qs 9s 3d
```

## Example output

```
$ theb play 10s js qs 9s 3d
Hole cards:
  Ten of Spades, Jack of Spades
Community cards:
  Queen of Spades, Nine of Spades, Three of Diamonds

Hand distribution after randomly drawing 2 community cards 100000 times:

  Flush                26.4%
  High card            12.6%
  One pair             24.9%
  Straight             19.4%
  Straight flush       8.3%
  Three of a kind      1.2%
  Two pair             7.2%

Opponent hand distribution after randomly drawing 2 community cards 100000 times:

  Flush                17.4%
  Four of a kind       0.8%
  Full house           17.2%
  High card            0.4%
  One pair             6.9%
  Straight             15.0%
  Straight flush       0.1%
  Three of a kind      2.6%
  Two pair             39.5%

Outcome distribution after playing against one opponent 100000 times:

  Loss                 57.4%
  Tie                  1.5%
  Win                  41.1%
```

## Quick start

You will need [Rust](https://www.rust-lang.org/).

```
cargo install --path .
theb --help
```
