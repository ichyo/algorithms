//! ref: [https://www.nayuki.io/page/next-lexicographical-permutation-algorithm]

pub trait Permutation {
    fn next_permutation(&mut self) -> bool;
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
