/// Simpler fixed length version of [bit-vec](https://docs.rs/bit-vec/)
pub struct BitArray {
    blocks: Vec<u64>,
    nbits: usize,
}

impl BitArray {
    pub fn new(nbits: usize) -> BitArray {
        let len = (nbits + 63) / 64;
        BitArray {
            blocks: vec![0; len],
            nbits,
        }
    }

    pub fn len(&self) -> usize {
        self.nbits
    }

    pub fn is_empty(&self) -> bool {
        self.nbits == 0
    }

    pub fn get(&self, i: usize) -> bool {
        assert!(
            i < self.nbits,
            "index out of bounds: {:?} >= {:?}",
            i,
            self.nbits
        );
        let w = i / 64;
        let b = i % 64;
        (self.blocks[w] & (1 << b)) != 0
    }

    pub fn set(&mut self, i: usize, x: bool) {
        assert!(
            i < self.nbits,
            "index out of bounds: {:?} >= {:?}",
            i,
            self.nbits
        );
        let w = i / 64;
        let b = i % 64;
        let flag = 1 << b;
        let val = if x {
            self.blocks[w] | flag
        } else {
            self.blocks[w] & !flag
        };
        self.blocks[w] = val;
    }

    pub fn union(&mut self, other: &Self) {
        self.mutate(other, |x, y| (x | y))
    }

    pub fn intersect(&mut self, other: &Self) {
        self.mutate(other, |x, y| (x & y))
    }

    pub fn difference(&mut self, other: &Self) {
        self.mutate(other, |x, y| (x & !y))
    }

    fn mutate<F>(&mut self, other: &Self, op: F)
    where
        F: Fn(u64, u64) -> u64,
    {
        assert_eq!(self.len(), other.len());
        for (a, b) in self.blocks_mut().zip(other.blocks()) {
            let w = op(*a, *b);
            *a = w;
        }
    }

    fn blocks(&self) -> impl Iterator<Item = &u64> {
        self.blocks.iter()
    }

    fn blocks_mut(&mut self) -> impl Iterator<Item = &mut u64> {
        self.blocks.iter_mut()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_and_set() {
        let mut bit = BitArray::new(5);
        assert_eq!(false, bit.get(0));
        assert_eq!(false, bit.get(2));
        assert_eq!(false, bit.get(4));
        bit.set(0, true);
        bit.set(1, true);
        bit.set(4, true);
        assert_eq!(true, bit.get(0));
        assert_eq!(false, bit.get(2));
        assert_eq!(true, bit.get(4));
    }

    #[test]
    fn test_get_and_set_100() {
        let mut bit = BitArray::new(100);
        for i in 0..100 {
            bit.set(i, i % 2 == 0);
        }
        for i in 0..100 {
            assert_eq!(i % 2 == 0, bit.get(i));
        }
    }

    #[test]
    fn test_intersect() {
        let mut bit1 = {
            let mut bit = BitArray::new(5);
            bit.set(0, true);
            bit.set(1, true);
            bit.set(4, true);
            bit
        };
        let bit2 = {
            let mut bit = BitArray::new(5);
            bit.set(0, true);
            bit.set(2, true);
            bit.set(4, true);
            bit
        };
        bit1.intersect(&bit2);
        assert_eq!(true, bit1.get(0));
        assert_eq!(false, bit1.get(1));
        assert_eq!(false, bit1.get(2));
        assert_eq!(false, bit1.get(3));
        assert_eq!(true, bit1.get(4));
    }

    #[test]
    fn test_union() {
        let mut bit1 = {
            let mut bit = BitArray::new(5);
            bit.set(0, true);
            bit.set(1, true);
            bit.set(4, true);
            bit
        };
        let bit2 = {
            let mut bit = BitArray::new(5);
            bit.set(0, true);
            bit.set(2, true);
            bit.set(4, true);
            bit
        };
        bit1.union(&bit2);
        assert_eq!(true, bit1.get(0));
        assert_eq!(true, bit1.get(1));
        assert_eq!(true, bit1.get(2));
        assert_eq!(false, bit1.get(3));
        assert_eq!(true, bit1.get(4));
    }

    #[test]
    fn test_difference() {
        let mut bit1 = {
            let mut bit = BitArray::new(5);
            bit.set(0, true);
            bit.set(1, true);
            bit.set(4, true);
            bit
        };
        let bit2 = {
            let mut bit = BitArray::new(5);
            bit.set(0, true);
            bit.set(2, true);
            bit.set(4, true);
            bit
        };
        bit1.difference(&bit2);
        assert_eq!(false, bit1.get(0));
        assert_eq!(true, bit1.get(1));
        assert_eq!(false, bit1.get(2));
        assert_eq!(false, bit1.get(3));
        assert_eq!(false, bit1.get(4));
    }
}
