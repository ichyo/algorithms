# rust-algorithms

[![Build Status](https://travis-ci.com/ichyo/rust-algorithms.svg?branch=master)](https://travis-ci.com/ichyo/rust-algorithms)

This is a collection of algorithms for programming contests such as codeforces and AtCoder.
This should compile with 1.15.1 so that you can use it with the old compiler in AtCoder.

Currently, it supports a few basic data structures and functions.
* Binary Indexed Tree
* Union-Find (Disjoint Set)
* ModInt (automatically computes mod N with several operations)
* Combinations
* Primality test (Miller-Rabin)
* Next/Prev Permutation

I suggest combining this library into single source file and pasting into the buttom of your submission file when you use it in programming contests.
You can try [rust-bundler](https://github.com/ichyo/rust-bundler) or other alternatives.
