//! data structures for efficient operations
//!
//! This module contains these data structures
//! * [`BIT`](struct.BIT.html) - Binary Index Tree (Fenwick Tree)
//! * [`BITRange`](struct.BITRange.html) - Binary Index Tree (Fenwick Tree) + range add
//! * [`UnionFind`](struct.UnionFind.html) - Disjoint-set (Union-find) data structure
//! * [`RMQ`](type.RMQ.html) - Segment tree to support range minimum query
//!
mod bit;
mod bit_array;
mod segment_tree;
mod union_find;

pub use self::bit::{BITRange, BIT};
pub use self::bit_array::*;
pub use self::segment_tree::{MinOp, Monoid, SegmentTree, RMQ};
pub use self::union_find::UnionFind;
