//! data structures for efficient operations
//!
//! Now, this module contains these data structures
//! * [`BIT`](struct.BIT.html) - Binary Index Tree (Fenwick Tree)
//! * [`BITRange`](struct.BITRange.html) - Binary Index Tree (Fenwick Tree) + range add
//! * [`UnionFind`](struct.UnionFind.html) - Disjoint-set (Union-find) data structure
//!
mod bit;
mod union_find;

pub use self::bit::{BITRange, BIT};
pub use self::union_find::UnionFind;
