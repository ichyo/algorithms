//! Segment tree to provide efficent range query
//!
//! Implement `Monoid` to write custom type of segment tree.
//! You can check how `RMQ` is implmented in this module.
// ref: https://github.com/asi1024/competitive-library/blob/master/cpp/include/structure/segment_tree.cpp
// ref: rust-num
use std::cmp;
use std::marker::PhantomData;

pub trait Monoid<T> {
    fn id() -> Option<T> {
        None
    }
    fn op(l: &Option<T>, r: &Option<T>) -> Option<T>;
}

pub struct MinOp<T: Ord> {
    phantom: PhantomData<T>,
}

impl<T: Ord + Clone> Monoid<T> for MinOp<T> {
    #[inline]
    fn op(l: &Option<T>, r: &Option<T>) -> Option<T> {
        match (l, r) {
            (Some(l), Some(r)) => Some(cmp::min(l, r).clone()),
            (Some(l), None) => Some(l.clone()),
            (None, Some(r)) => Some(r.clone()),
            (None, None) => None,
        }
    }
}

pub struct MaxOp<T: Ord> {
    phantom: PhantomData<T>,
}

impl<T: Ord + Clone> Monoid<T> for MaxOp<T> {
    #[inline]
    fn op(l: &Option<T>, r: &Option<T>) -> Option<T> {
        match (l, r) {
            (Some(l), Some(r)) => Some(cmp::max(l, r).clone()),
            (Some(l), None) => Some(l.clone()),
            (None, Some(r)) => Some(r.clone()),
            (None, None) => None,
        }
    }
}

pub struct SegmentTree<M: Monoid<T>, T: Clone> {
    phantom: PhantomData<M>,
    data: Vec<Option<T>>,
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
        let mut data = vec![None; size_p2 * 2];
        for (i, x) in v.into_iter().enumerate() {
            data[size_p2 + i] = Some(x);
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
        self.data[pos] = Some(value);
        loop {
            pos /= 2;
            if pos == 0 {
                break;
            }
            self.data[pos] = M::op(&self.data[pos * 2], &self.data[pos * 2 + 1]);
        }
    }

    pub fn query(&self, mut l: usize, mut r: usize) -> Option<T> {
        assert!(l <= r && r <= self.size);
        l += self.size_p2;
        r += self.size_p2;
        let mut res1 = M::id();
        let mut res2 = M::id();
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
pub type RMQ<T> = SegmentTree<MinOp<T>, Option<T>>;

#[cfg(test)]
mod tests {
    use super::RMQ;

    #[test]
    fn test_rms() {
        let mut test = vec![1, 5, 4, 8, 6, 9, 2, 0, 8, 1];
        let mut rms = RMQ::from_vec(test.clone());
        for i in 0..test.len() {
            for j in i..test.len() + 1 {
                assert_eq!(test[i..j].iter().cloned().min(), rms.query(i, j));
            }
        }
        rms.update(7, 5);
        test[7] = 5;
        for i in 0..test.len() {
            for j in i..test.len() + 1 {
                assert_eq!(test[i..j].iter().cloned().min(), rms.query(i, j));
            }
        }
    }
}
