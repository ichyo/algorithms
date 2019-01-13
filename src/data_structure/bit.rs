// cf. http://hos.ac/slides/20140319_bit.pdf

/// Binary Indexed Tree (Fenwick Tree) (0-indexed)
///
/// This data structure supports these two queries in O(log n)
///
/// 1. add w to v[at]
/// 2. the sum of v[begin], v[begin+1], .., v[end-1]
pub struct BIT {
    tree: Vec<i64>,
}

/// BIT (range-version) (0-indexed)
///
/// This data structure two queries in O(log n)
///
/// 1. add w to v[begin], v[begin+1], ..., v[end-1]
/// 2. get the sum of v[begin], v[begin+1], ..., v[end-1]
pub struct BITRange {
    bit0: BIT,
    bit1: BIT,
}

impl BIT {
    pub fn new(n: usize) -> BIT {
        BIT { tree: vec![0; n] }
    }

    /// v[at] += by
    pub fn add(&mut self, at: usize, by: i64) {
        let mut idx = at;
        while idx < self.tree.len() {
            self.tree[idx] += by;
            idx |= idx + 1;
        }
    }

    /// sum of v[idx] such that begin <= idx < end
    pub fn get(&self, begin: usize, end: usize) -> i64 {
        if begin >= end {
            return 0;
        }
        let a = if end > 0 { self.cum(end - 1) } else { 0 };
        let b = if begin > 0 { self.cum(begin - 1) } else { 0 };
        a - b
    }

    /// v[0] + ... + v[last]
    fn cum(&self, last: usize) -> i64 {
        let mut res = 0;
        let mut idx = last as i64;
        while idx >= 0 {
            res += self.tree[idx as usize];
            idx = (idx & (idx + 1)) - 1;
        }
        res
    }
}

impl BITRange {
    pub fn new(n: usize) -> BITRange {
        BITRange {
            bit0: BIT::new(n + 1),
            bit1: BIT::new(n + 1),
        }
    }

    /// v[begin], v[begin+1], ..., v[end-1] += by
    pub fn add(&mut self, begin: usize, end: usize, by: i64) {
        let a = begin as i64;
        let b = end as i64;
        self.bit0.add(begin, -by * a);
        self.bit0.add(end, by * b);
        self.bit1.add(begin, by);
        self.bit1.add(end, -by);
    }

    /// v[begin] + ... + v[end-1]
    pub fn get(&self, begin: usize, end: usize) -> i64 {
        if begin >= end {
            return 0;
        }
        self.cum(end) - self.cum(begin)
    }

    /// v[0] + ... + v[end-1]
    fn cum(&self, end: usize) -> i64 {
        let a = self.bit0.get(0, end);
        let b = self.bit1.get(0, end);
        return a + b * end as i64;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple() {
        let mut bit = BIT::new(10);
        bit.add(2, 1);
        bit.add(3, 3);
        bit.add(5, 10);
        bit.add(0, -4);
        bit.add(9, -5);
        assert_eq!(5, bit.get(0, 10));
        assert_eq!(4, bit.get(2, 4));
        assert_eq!(1, bit.get(2, 3));
        assert_eq!(0, bit.get(2, 2));
        assert_eq!(0, bit.get(2, 0));
        assert_eq!(-4, bit.get(0, 1));
        assert_eq!(5, bit.get(5, 10));

        let mut bit = BITRange::new(10);
        bit.add(1, 3, 1);
        bit.add(2, 5, 2);
        assert_eq!(1, bit.get(1, 2));
        assert_eq!(4, bit.get(1, 3));
        assert_eq!(6, bit.get(1, 4));
        assert_eq!(6, bit.get(0, 4));
    }
}
