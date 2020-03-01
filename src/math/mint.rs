use std::fmt;
use std::marker::PhantomData;
use std::mem;
use std::ops;

/// Trait for `Mint`. `module()` should return prime number.
pub trait Module: Copy + Clone {
    fn module() -> u32;
}

/// One of famous numbers in programming contest. `10^9 + 7`
pub const MOD_107: u32 = 1_000_000_007;
/// One of famous numbers in programming contest. `10^9 + 9`
pub const MOD_109: u32 = 1_000_000_009;
/// One of famous numbers in programming contest. `998_244_353`
pub const MOD_998: u32 = 998_244_353;

/// struct to implement Module trait. it returns `MOD_107`.
#[derive(Debug, Copy, Clone)]
pub struct Mod107;
impl Module for Mod107 {
    fn module() -> u32 {
        MOD_107
    }
}

/// struct to implement Module trait. it returns `MOD_109`.
#[derive(Debug, Copy, Clone)]
pub struct Mod109;
impl Module for Mod109 {
    fn module() -> u32 {
        MOD_109
    }
}

/// struct to implement Module trait. it returns `MOD_998`.
#[derive(Debug, Copy, Clone)]
pub struct Mod998;
impl Module for Mod998 {
    fn module() -> u32 {
        MOD_998
    }
}

/// Wrapper class to compute mod `1_000_000_007` automatically.
///
/// # Examples
/// ```
/// use algorithms::math::{Mint107, MOD_107};
/// let x: Mint107 = 1234567.into();
/// let y: Mint107 = 2345678.into();
/// let z = x * y;
/// # // TODO: implement convert to u64
/// assert_eq!(z.val as u64, 1234567u64 * 2345678u64 % MOD_107 as u64);
/// ```
///
pub type Mint107 = Mint<Mod107>;

/// Wrapper class to compute mod `1_000_000_009` automatically.
///
/// # Examples
/// ```
/// use algorithms::math::{Mint109, MOD_109};
/// let x: Mint109 = 1234567.into();
/// let y: Mint109 = 2345678.into();
/// let z = x * y;
/// assert_eq!(z.val as u64, 1234567u64 * 2345678u64 % MOD_109 as u64);
/// ```
///
pub type Mint109 = Mint<Mod109>;

/// Wrapper class to compute mod `998_244_353` automatically.
///
/// # Examples
/// ```
/// use algorithms::math::{Mint998, MOD_998};
/// let x: Mint998 = 1234567.into();
/// let y: Mint998 = 2345678.into();
/// let z = x * y;
/// assert_eq!(z.val as u64, 1234567u64 * 2345678u64 % MOD_998 as u64);
/// ```
///
pub type Mint998 = Mint<Mod998>;

/// Wrapper class to compute modulo operation.
/// See examples
/// [`Mint107`](type.Mint107.html),
/// [`Mint109`](type.Mint109.html),
/// [`Mint998`](type.Mint998.html)
///
/// # Examples
/// ```
/// use algorithms::math::{Mint107, MOD_107};
/// let x: Mint107 = 1234567.into();
/// let y: Mint107 = 2345678.into();
/// let z = x * y;
/// assert_eq!(z.val as u64, 1234567u64 * 2345678u64 % MOD_107 as u64);
/// ```
#[derive(Debug, Copy, Clone, Eq)]
pub struct Mint<M: Module> {
    /// internal value. this is always less than `self.module()`.
    pub val: u32,
    m: PhantomData<M>,
}

impl<M: Module> Mint<M> {
    fn module(self) -> u32 {
        M::module()
    }
    fn new(val: u32) -> Mint<M> {
        assert!(val < M::module());
        Mint {
            val: val,
            m: PhantomData,
        }
    }
}

impl<T: Into<Mint<M>>, M: Module> ops::Add<T> for Mint<M> {
    type Output = Mint<M>;

    fn add(self, other: T) -> Mint<M> {
        let nval = self.val + other.into().val;
        Mint::new(if nval >= self.module() {
            nval - self.module()
        } else {
            nval
        })
    }
}

impl<T: Into<Mint<M>>, M: Module> ops::Sub<T> for Mint<M> {
    type Output = Mint<M>;

    fn sub(self, other: T) -> Mint<M> {
        let nval = self.val + self.module() - other.into().val;
        Mint::new(if nval >= self.module() {
            nval - self.module()
        } else {
            nval
        })
    }
}

impl<T: Into<Mint<M>>, M: Module> ops::Mul<T> for Mint<M> {
    type Output = Mint<M>;

    fn mul(self, other: T) -> Mint<M> {
        let nval = self.val as u64 * other.into().val as u64;
        Mint::new((nval % (self.module() as u64)) as u32)
    }
}

impl<T: Into<Mint<M>>, M: Module> ops::Div<T> for Mint<M> {
    type Output = Mint<M>;

    fn div(self, other: T) -> Mint<M> {
        self * other.into().inv()
    }
}

impl<M: Module> Mint<M> {
    /// Returns number `y` that satisfies `x * y == 1` where `x` is the original value.
    ///
    /// This assumes `module()` returns prime number.
    pub fn inv(self) -> Mint<M> {
        let mut a = self.val as i32;
        let mut b = self.module() as i32;
        let mut u = 1 as i32;
        let mut v = 0 as i32;
        while b != 0 {
            let t = a / b;
            a -= t * b;
            mem::swap(&mut a, &mut b);
            u -= t * v;
            mem::swap(&mut u, &mut v);
        }
        Mint::new(if u < 0 { u + self.module() as i32 } else { u } as u32)
    }
}

impl<M: Module> PartialEq for Mint<M> {
    fn eq(&self, other: &Mint<M>) -> bool {
        self.val == other.val
    }
}

impl<M: Module> fmt::Display for Mint<M> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.val.fmt(f)
    }
}

macro_rules! impl_signed_mint {
    ($($t:ty)*) => ($(
        impl<M: Module> From<$t> for Mint<M> {
            #[inline]
            fn from(x: $t) -> Mint<M> {
                let t = (x as i64) % (M::module() as i64);
                if x >= 0 {
                    Mint{ val: t as u32, m: PhantomData }
                } else {
                    Mint{ val: (M::module() as i64 + t) as u32, m: PhantomData }
                }
            }
        }
    )*)
}

macro_rules! impl_unsigned_mint {
    ($($t:ty)*) => ($(
        impl<M: Module> From<$t> for Mint<M> {
            #[inline]
            fn from(x: $t) -> Mint<M> {
                let t = x as u64 % M::module() as u64;
                Mint::new(t as u32)
            }
        }
    )*)
}

impl_signed_mint! { i8 i16 i32 i64 isize }
impl_unsigned_mint! { u8 u16 u32 u64 usize }

impl<T: Into<Mint<M>>, M: Module> ops::AddAssign<T> for Mint<M> {
    fn add_assign(&mut self, other: T) {
        *self = *self + other.into();
    }
}

impl<T: Into<Mint<M>>, M: Module> ops::SubAssign<T> for Mint<M> {
    fn sub_assign(&mut self, other: T) {
        *self = *self - other.into();
    }
}

impl<T: Into<Mint<M>>, M: Module> ops::MulAssign<T> for Mint<M> {
    fn mul_assign(&mut self, other: T) {
        *self = *self * other.into();
    }
}

impl<T: Into<Mint<M>>, M: Module> ops::DivAssign<T> for Mint<M> {
    fn div_assign(&mut self, other: T) {
        *self = *self / other.into();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple() {
        let a: Mint<Mod107> = Mint::from(3);
        let b: Mint<Mod107> = Mint::from(1000000000);
        assert_eq!(Mint::from(3000000000u64 % Mod107::module() as u64), a * b);
    }
}
