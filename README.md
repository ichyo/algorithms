# Algorithms
[![Build](https://github.com/ichyo/algorithms/workflows/Build/badge.svg)](https://github.com/ichyo/algorithms/actions?query=workflow%3ABuild)
[![Verify](https://github.com/ichyo/algorithms/workflows/Verify/badge.svg)](https://github.com/ichyo/algorithms/actions?query=workflow%3AVerify)

This repository manages a collection of classic algorithms for programming contests (e.g. [codeforces](https://codeforces.com/) and [AtCoder](https://atcoder.jp/)).

Minimum supported compiler version is 1.35.0. Use [cargo-atcoder](https://github.com/tanakh/cargo-atcoder) for AtCoder.

## List of algorithms

### Data Structures

* [BIT (Binary Indexed Tree)](src/data_structure/bit.rs)
* [Union-Find (Disjoint Set)](src/data_structure/union_find.rs)
* [Segment Tree](src/data_structure/segment_tree.rs)

### Math

* [ModInt](src/math/mint.rs)
* [Combinations](src/math/comb.rs)

### Others

* [Binary Search](src/util/binary_search.rs)
* [Next/Prev Permutation](src/util/permutation.rs)
* [Xorshift random generator](src/util/random.rs)
