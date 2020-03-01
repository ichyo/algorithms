// cf. http://hos.ac/slides/20140319_bit.pdf

/// Binary Indexed Tree (0-indexed)
///
/// This data structure supports these two queries in O(log n)
///
/// 1. add w to v[at]
/// 2. the sum of v[begin], v[begin+1], .., v[end-1]
pub struct BIT {
    tree: Vec<i64>,
}

/// Binary Indexed Tree (range-version) (0-indexed)
///
/// This data structure two queries in O(log n)
///
/// 1. add w to v[begin], v[begin+1], ..., v[end-1]
/// 2. get the sum of v[begin], v[begin+1], ..., v[end-1]
pub struct BITRange {
    bit0: BIT,
    bit1: BIT,
}

#[allow(clippy::len_without_is_empty)] // because empty BIT doesn't make sense
impl BIT {
    /// Constructs a new BIT of length `len`.
    /// All values are initialized zero.
    ///
    /// # Examples
    /// ```
    /// use algorithms::data_structure::BIT;
    /// # #[warn(unused_mut)]
    /// let mut bit = BIT::new(100);
    /// ```
    pub fn new(len: usize) -> BIT {
        BIT { tree: vec![0; len] }
    }

    /// Returns the number of elements in the BIT.
    ///
    /// # Examples
    ///
    /// ```
    /// use algorithms::data_structure::BIT;
    /// let mut bit = BIT::new(100);
    /// assert_eq!(bit.len(), 100);
    /// ```
    pub fn len(&self) -> usize {
        self.tree.len()
    }

    /// Add a value `value` to a element of index `index`.
    /// v[index] += value
    ///
    /// # Panics
    ///
    /// Panics if `index > len`
    ///
    /// # Examples
    ///
    /// ```
    /// use algorithms::data_structure::BIT;
    /// let mut bit = BIT::new(10);
    /// bit.add(5, 100);
    /// assert_eq!(bit.get(3, 6), 100);
    /// bit.add(5, 10);
    /// assert_eq!(bit.get(3, 6), 110);
    /// ```
    pub fn add(&mut self, index: usize, value: i64) {
        assert!(index < self.tree.len());
        let mut idx = index;
        while idx < self.tree.len() {
            self.tree[idx] += value;
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
        assert!(last < self.tree.len());
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
        if begin >= end {
            return;
        }
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
        a + b * end as i64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn test_out_of_bound_add() {
        let mut bit = BIT::new(10);
        bit.add(100, 1);
    }

    #[test]
    #[should_panic]
    fn test_out_of_bound_range_add() {
        let mut bit = BITRange::new(10);
        bit.add(5, 100, 1);
    }

    #[test]
    #[should_panic]
    fn test_out_of_bound_get() {
        let bit = BIT::new(10);
        bit.get(0, 1000);
    }

    #[test]
    fn test_empty_range() {
        let mut bit = BITRange::new(10);
        bit.add(9, 0, 100);
        bit.add(7, 3, 100);
        assert_eq!(0, bit.get(0, 9));
        assert_eq!(0, bit.get(0, 5));
        assert_eq!(0, bit.get(2, 5));
    }

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
