//! mathematic related functions and structs
//!
//! * [`Comb`](struct.Comb.html) - calculate combinations
//! * [`Mint`](struct.Mint.html), [`Mint107`](type.Mint107.html), [`Mint109`](type.Mint109.html),
//! [`Mint998`](type.Mint998.html) - wrapper of integer that automatically call modulo operations

mod comb;
mod mint;

pub use self::comb::Comb;
pub use self::mint::{Mint, Module};
pub use self::mint::{Mint107, Mint109, Mint998};
pub use self::mint::{Mod107, Mod109, Mod998};
pub use self::mint::{MOD_107, MOD_109, MOD_998};
