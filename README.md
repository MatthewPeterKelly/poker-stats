# Poker Stats with Rust

This small project is primarily a way for me to learn rust. The idea is to use Monte-Carlo simulation to compute the probability of drawing each of the different scoring hands. These probabilities can easily be computed analytically, so we have a good way to verify the results of the program.

## Usage Examples:

Show top-level help file for the command-line interface:
```
cargo run -- --help
```

Quick default demonstration: draws a five card hand and scores it:
```
cargo run
```

Run statistics for drawing 5-card hands, using 100000 samples and 8 threads:

```
cargo run -- statistics 5 100000000
```
```
HandScore:
  high_card         100000000 (100.000%)
  pair              49287061  ( 49.287%)
  two_pair          4755262   (  4.755%)
  three_of_a_kind   2278276   (  2.278%)
  straight          354474    (  0.354%)
  flush             198043    (  0.198%)
  full_house        142993    (  0.143%)
  four_of_a_kind    23990     (  0.024%)
  straight_flush    1365      (  0.001%)
```

Draw a single 7-card hand:
```
cargo run -- draw-hand 7
```
```
Hand: 9♣, 8♥, 4♥, 6♦, 2♣, 2♦, 8♣
HandStats:
  Count: 7
  Suits: [♣]: 3, [♦]: 2, [♥]: 2
  Ranks: [2]: 2, [4]: 1, [6]: 1, [8]: 2, [9]: 1
HandScore:
  high_card         1 (100.000%)
  pair              1 (100.000%)
  two_pair          1 (100.000%)
  three_of_a_kind   0 (  0.000%)
  straight          0 (  0.000%)
  flush             0 (  0.000%)
  full_house        0 (  0.000%)
  four_of_a_kind    0 (  0.000%)
  straight_flush    0 (  0.000%)
```

## Design:

My overall goal is learning, so I've tried to write idiomatic Rust code as much as possible. I'm coming from a C++ background, so perhaps I missed a few things.

I've tried to make the "inner loop" code as efficient as possible, with a side goal of optimizing the time to draw and score each hand. So far I've mostly approached this from a design standpoint, rather than using real profiling tools.

## Organization:

This repository is set up with a small library that provides all of the classes and utility functions needed to run the analysis, along with a simple main function that exercises the library. Each file in the library includes unit tests.

## Contributing

If you're interested in contributing to this project just send me and email or open a PR! This could include new features, bug-fixes, implementations for suggestions in the issues, or just style improvements to make the code more idiomatic.
