//! Segment tree to provide efficent range query
//!
//! Implement `Monoid` to write custom type of segment tree.
//! You can check how `RMQ` is implmented in this module.
// ref: https://github.com/asi1024/competitive-library/blob/master/cpp/include/structure/segment_tree.cpp
// ref: rust-num
use std::cmp;
use std::marker::PhantomData;
use std::{f32, f64};
use std::{i16, i32, i64, i8, isize};
use std::{u16, u32, u64, u8, usize};

pub trait Monoid<T> {
    fn id() -> T;
    fn op(l: &T, r: &T) -> T;
}

pub trait Bounded {
    fn min_value() -> Self;
    fn max_value() -> Self;
}

macro_rules! bounded_impl {
    ($t:ty, $min:expr, $max:expr) => {
        impl Bounded for $t {
            #[inline]
            fn min_value() -> $t {
                $min
            }

            #[inline]
            fn max_value() -> $t {
                $max
            }
        }
    };
}

bounded_impl!(usize, usize::MIN, usize::MAX);
bounded_impl!(u8, u8::MIN, u8::MAX);
bounded_impl!(u16, u16::MIN, u16::MAX);
bounded_impl!(u32, u32::MIN, u32::MAX);
bounded_impl!(u64, u64::MIN, u64::MAX);

bounded_impl!(isize, isize::MIN, isize::MAX);
bounded_impl!(i8, i8::MIN, i8::MAX);
bounded_impl!(i16, i16::MIN, i16::MAX);
bounded_impl!(i32, i32::MIN, i32::MAX);
bounded_impl!(i64, i64::MIN, i64::MAX);

bounded_impl!(f32, f32::MIN, f32::MAX);
bounded_impl!(f64, f64::MIN, f64::MAX);

macro_rules! for_each_tuple_ {
    ( $m:ident !! ) => (
        $m! { }
    );
    ( $m:ident !! $h:ident, $($t:ident,)* ) => (
        $m! { $h $($t)* }
        for_each_tuple_! { $m !! $($t,)* }
    );
}

macro_rules! for_each_tuple {
    ( $m:ident ) => {
        for_each_tuple_! { $m !! A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, }
    };
}

macro_rules! bounded_tuple {
    ( $($name:ident)* ) => (
        impl<$($name: Bounded,)*> Bounded for ($($name,)*) {
            #[inline]
            fn min_value() -> Self {
                ($($name::min_value(),)*)
            }
            #[inline]
            fn max_value() -> Self {
                ($($name::max_value(),)*)
            }
        }
    );
}

for_each_tuple!(bounded_tuple);

pub struct RMQOp<T: Ord + Bounded> {
    phantom: PhantomData<T>,
}

impl<T: Ord + Bounded + Copy> Monoid<T> for RMQOp<T> {
    #[inline]
    fn id() -> T {
        T::max_value()
    }

    #[inline]
    fn op(l: &T, r: &T) -> T {
        *cmp::min(l, r)
    }
}

pub struct SegmentTree<M: Monoid<T>, T: Clone> {
    phantom: PhantomData<M>,
    data: Vec<T>,
    size: usize,
    size_p2: usize,
}

impl<M: Monoid<T>, T: Clone> SegmentTree<M, T> {
    pub fn from_vec(v: Vec<T>) -> SegmentTree<M, T> {
        let size = v.len();
        let mut size_p2 = 1;
        while size_p2 < v.len() {
            size_p2 *= 2;
        }
        let mut data = vec![M::id(); size_p2 * 2];
        for (i, x) in v.into_iter().enumerate() {
            data[size_p2 + i] = x;
        }
        for i in (0..size_p2).rev() {
            data[i] = M::op(&data[i * 2 + 0], &data[i * 2 + 1]);
        }
        SegmentTree {
            phantom: PhantomData,
            data: data,
            size: size,
            size_p2: size_p2,
        }
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn update(&mut self, mut pos: usize, value: T) {
        assert!(pos < self.size);
        pos += self.size_p2;
        self.data[pos] = value;
        loop {
            pos /= 2;
            if pos == 0 {
                break;
            }
            self.data[pos] = M::op(&self.data[pos * 2], &self.data[pos * 2 + 1]);
        }
    }

    pub fn query(&self, mut l: usize, mut r: usize) -> T {
        assert!(l <= r && r <= self.size);
        l += self.size_p2;
        r += self.size_p2;
        let mut res1: T = M::id();
        let mut res2: T = M::id();
        while l < r {
            if (l & 1) != 0 {
                res1 = M::op(&res1, &self.data[l]);
                l += 1;
            }
            if (r & 1) != 0 {
                r -= 1;
                res2 = M::op(&self.data[r], &res2);
            }
            l >>= 1;
            r >>= 1;
        }
        M::op(&res1, &res2)
    }
}

/// segment tree to get minimum value in a range
pub type RMQ<T> = SegmentTree<RMQOp<T>, T>;

#[cfg(test)]
mod tests {
    use super::RMQ;

    #[test]
    fn test_rms() {
        let test = vec![1, 5, 4, 8, 6, 9, 2, 0];
        let mut rms = RMQ::from_vec(test);
        assert_eq!(1, rms.query(0, 1));
        assert_eq!(1, rms.query(0, 2));
        assert_eq!(1, rms.query(0, 3));
        assert_eq!(1, rms.query(0, 4));
        assert_eq!(1, rms.query(0, 4));
        assert_eq!(1, rms.query(0, 5));
        assert_eq!(1, rms.query(0, 6));
        assert_eq!(1, rms.query(0, 7));
        assert_eq!(0, rms.query(0, 8));
        assert_eq!(5, rms.query(1, 2));
        assert_eq!(4, rms.query(1, 3));
        assert_eq!(4, rms.query(1, 4));
        assert_eq!(4, rms.query(1, 6));
        assert_eq!(2, rms.query(1, 7));
        assert_eq!(0, rms.query(1, 8));
        assert_eq!(2, rms.query(6, 7));
        assert_eq!(0, rms.query(7, 8));
        rms.update(7, 5);
        assert_eq!(1, rms.query(0, 8));
        assert_eq!(5, rms.query(1, 2));
        assert_eq!(4, rms.query(1, 3));
        assert_eq!(4, rms.query(1, 4));
        assert_eq!(4, rms.query(1, 6));
        assert_eq!(2, rms.query(1, 7));
        assert_eq!(2, rms.query(1, 8));
        assert_eq!(2, rms.query(6, 7));
        assert_eq!(5, rms.query(7, 8));
    }
}
