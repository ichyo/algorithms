use std;

pub const MOD: u32 = 1000000007;

//TODO: It has to be generics on MOD value.
//TODO: https://github.com/rust-lang/rust/issues/44580 may be releated
#[derive(Debug, Copy, Clone, Eq)]
pub struct Mint {
    pub val: u32,
}

impl<T: Into<Mint>> std::ops::Add<T> for Mint {
    type Output = Mint;

    fn add(self, other: T) -> Mint {
        let nval = self.val + other.into().val;
        Mint {
            val: if nval >= MOD { nval - MOD } else { nval },
        }
    }
}

impl<T: Into<Mint>> std::ops::Sub<T> for Mint {
    type Output = Mint;

    fn sub(self, other: T) -> Mint {
        let nval = self.val + MOD - other.into().val;
        Mint {
            val: if nval >= MOD { nval - MOD } else { nval },
        }
    }
}

impl<T: Into<Mint>> std::ops::Mul<T> for Mint {
    type Output = Mint;

    fn mul(self, other: T) -> Mint {
        let nval = self.val as u64 * other.into().val as u64;
        Mint {
            val: (nval % (MOD as u64)) as u32,
        }
    }
}

impl<T: Into<Mint>> std::ops::Div<T> for Mint {
    type Output = Mint;

    fn div(self, other: T) -> Mint {
        self * other.into().inv()
    }
}

impl Mint {
    pub fn inv(self) -> Mint {
        let mut a = self.val as i32;
        let mut b = MOD as i32;
        let mut u = 1 as i32;
        let mut v = 0 as i32;
        while b != 0 {
            let t = a / b;
            a -= t * b;
            std::mem::swap(&mut a, &mut b);
            u -= t * v;
            std::mem::swap(&mut u, &mut v);
        }
        Mint {
            val: if u < 0 { u + MOD as i32 } else { u } as u32,
        }
    }
}

impl PartialEq for Mint {
    fn eq(&self, other: &Mint) -> bool {
        self.val == other.val
    }
}

impl std::fmt::Display for Mint {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.val.fmt(f)
    }
}

macro_rules! impl_signed_mint {
    ($($t:ty)*) => ($(
        impl From<$t> for Mint {
            #[inline]
            fn from(x: $t) -> Mint {
                let t = (x as i64) % (MOD as i64);
                if x >= 0 {
                    Mint{ val: t as u32 }
                } else {
                    Mint{ val: (MOD as i64 + t) as u32 }
                }
            }
        }
    )*)
}

macro_rules! impl_unsigned_mint {
    ($($t:ty)*) => ($(
        impl From<$t> for Mint {
            #[inline]
            fn from(x: $t) -> Mint {
                let t = x as u64 % MOD as u64;
                Mint{ val: t as u32 }
            }
        }
    )*)
}

impl_signed_mint! { i8 i16 i32 i64 isize }
impl_unsigned_mint! { u8 u16 u32 u64 usize }

impl<T: Into<Mint>> std::ops::AddAssign<T> for Mint {
    fn add_assign(&mut self, other: T) {
        *self = *self + other.into();
    }
}

impl<T: Into<Mint>> std::ops::SubAssign<T> for Mint {
    fn sub_assign(&mut self, other: T) {
        *self = *self - other.into();
    }
}

impl<T: Into<Mint>> std::ops::MulAssign<T> for Mint {
    fn mul_assign(&mut self, other: T) {
        *self = *self * other.into();
    }
}

impl<T: Into<Mint>> std::ops::DivAssign<T> for Mint {
    fn div_assign(&mut self, other: T) {
        *self = *self / other.into();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple() {
        let a: Mint = Mint::from(3);
        let b: Mint = Mint::from(1000000000);
        assert_eq!(Mint::from(3000000000u64 % MOD as u64), a * b);
    }
}
