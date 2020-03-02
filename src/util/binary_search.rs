pub trait BinarySearch {
    type Item;

    /// Returns the index of the first element in the sorted slice
    /// which is greater or equal to the value.
    fn lower_bound(&self, value: &Self::Item) -> usize;

    /// Returns the index of the first element in the sorted slice
    /// which is greater to the value.
    fn upper_bound(&self, value: &Self::Item) -> usize;
}

impl<T> BinarySearch for [T]
where
    T: Ord,
{
    type Item = T;

    fn lower_bound(&self, value: &T) -> usize {
        if self.is_empty() {
            return 0;
        }
        if &self[0] >= value {
            return 0;
        }

        let mut lb = 0;
        let mut ub = self.len();

        // The last element x s.t x < value is always in [lb, ub)
        while ub - lb > 1 {
            let mid = lb + (ub - lb) / 2;
            if &self[mid] < value {
                lb = mid;
            } else {
                ub = mid;
            }
        }

        lb + 1
    }

    fn upper_bound(&self, value: &T) -> usize {
        if self.is_empty() {
            return 0;
        }
        if &self[0] > value {
            return 0;
        }

        let mut lb = 0;
        let mut ub = self.len();

        // The last element x s.t x <= value is always in [lb, ub)
        while ub - lb > 1 {
            let mid = lb + (ub - lb) / 2;
            if &self[mid] <= value {
                lb = mid;
            } else {
                ub = mid;
            }
        }

        lb + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lower_bound() {
        let data = [0, 0, 1, 1, 2];
        assert_eq!(0, data.lower_bound(&-1));
        assert_eq!(0, data.lower_bound(&0));
        assert_eq!(2, data.lower_bound(&1));
        assert_eq!(4, data.lower_bound(&2));
        assert_eq!(5, data.lower_bound(&3));
    }

    #[test]
    fn test_upper_bound() {
        let data = [0, 0, 1, 1, 2];
        assert_eq!(0, data.upper_bound(&-1));
        assert_eq!(2, data.upper_bound(&0));
        assert_eq!(4, data.upper_bound(&1));
        assert_eq!(5, data.upper_bound(&2));
        assert_eq!(5, data.upper_bound(&3));
    }
}
