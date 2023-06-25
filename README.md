# Poker Stats with Rust

This small project is primarily a way for me to learn rust. The idea is to use Monte-Carlo simulation to compute the probability of drawing each of the different scoring hands. These probabilities can easily be computed analytically, so we have a good way to verify the results of the program.

## Design:

My overall goal is learning, so I've tried to write idiomatic Rust code as much as possible. I'm coming from a C++ background, so perhaps I missed a few things.

I've tried to make the "inner loop" code as efficient as possible, with a side goal of optimizing the time to draw and score each hand. So far I've mostly approached this from a design standpoint, rather than using real profiling tools.

## Organization:

This repository is set up with a small library that provides all of the classes and utility functions needed to run the analysis, along with a simple main function that exercises the library. Each file in the library includes unit tests.

## Contributing

If you're interested in contributing to this project just send me and email or open a PR! This could include new features, bug-fixes, implementations for suggestions in the issues, or just style improvements to make the code more idiomatic.
