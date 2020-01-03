//! util functions and trails to help typical implementation
//!
//! * [`Permutation`](trait.Permutation.html) - Add `next_permutation`, `prev_permutation` for slices.
//! * [`XorShift`](struct.XorShift.html) - Provide very simple random generator.
//!
mod permutation;
mod random;
mod grid;

pub use self::permutation::Permutation;
pub use self::random::XorShift;
pub use self::grid::adj4_iter;
