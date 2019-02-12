// ref: [https://www.nayuki.io/page/next-lexicographical-permutation-algorithm]

/// Same method as `std::next_permutaion` in C++.
/// it rearranges the elements into the next/prev lexicographically greater permutation.
///
/// # Examples
/// ```
/// use algonium::util::Permutation;
///
/// let mut perm = [5, 0, 9, 2];
/// assert!(perm.next_permutation());
/// assert_eq!(perm, [5, 2, 0, 9]);
/// assert!(perm.next_permutation());
/// assert_eq!(perm, [5, 2, 9, 0]);
/// assert!(perm.prev_permutation());
/// assert_eq!(perm, [5, 2, 0, 9]);
///
/// let mut perm = [4, 3, 2, 1, 0];
/// assert!(!perm.next_permutation());
/// assert_eq!(perm, [4, 3, 2, 1, 0]);
/// ```
pub trait Permutation {
    /// It rearranges the elements into the next lexicographically greater permutation.
    ///
    /// Returns `false` without modifying any state only if current is the last permutation.
    fn next_permutation(&mut self) -> bool;

    /// It rearranges the elements into the previous lexicographically greater permutation.
    ///
    /// Returns `false` without modifying any state only if current is the first permutation.
    fn prev_permutation(&mut self) -> bool;
}

impl<T> Permutation for [T]
where
    T: Ord,
{
    fn next_permutation(&mut self) -> bool {
        if self.len() <= 1 {
            return false;
        }

        let mut i = self.len() - 1;
        while i > 0 && self[i - 1] >= self[i] {
            i -= 1;
        }

        if i == 0 {
            return false;
        }

        let mut j = self.len() - 1;
        while self[j] <= self[i - 1] {
            j -= 1;
        }

        self.swap(j, i - 1);

        self[i..].reverse();

        true
    }

    fn prev_permutation(&mut self) -> bool {
        if self.len() <= 1 {
            return false;
        }

        let mut i = self.len() - 1;
        while i > 0 && self[i - 1] <= self[i] {
            i -= 1;
        }

        if i == 0 {
            return false;
        }

        self[i..].reverse();

        let mut j = self.len() - 1;
        while self[j - 1] < self[i - 1] {
            j -= 1;
        }

        self.swap(i - 1, j);

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple() {
        let mut data = [0, 0, 1, 1, 2];
        data.next_permutation();
        assert_eq!(&data, &[0, 0, 1, 2, 1]);
        data.next_permutation();
        assert_eq!(&data, &[0, 0, 2, 1, 1]);
        data.next_permutation();
        assert_eq!(&data, &[0, 1, 0, 1, 2]);
        data.next_permutation();
        assert_eq!(&data, &[0, 1, 0, 2, 1]);
        data.next_permutation();
        assert_eq!(&data, &[0, 1, 1, 0, 2]);
        data.prev_permutation();
        assert_eq!(&data, &[0, 1, 0, 2, 1]);
        data.prev_permutation();
        assert_eq!(&data, &[0, 1, 0, 1, 2]);
        data.prev_permutation();
        assert_eq!(&data, &[0, 0, 2, 1, 1]);
        data.prev_permutation();
        assert_eq!(&data, &[0, 0, 1, 2, 1]);
        data.prev_permutation();
        assert_eq!(&data, &[0, 0, 1, 1, 2]);
        assert!(!data.prev_permutation());
    }
}
