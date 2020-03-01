use std::num::Wrapping;

/// Not secure but fast random generator.
///
/// # Examples
/// ```
/// use algorithms::util::XorShift;
/// let mut xorshift = XorShift::new();
/// assert_eq!(3701687786, xorshift.next_u32());
/// assert_eq!(458299110, xorshift.next_u32());
/// ```
#[derive(Clone, Debug, Default)]
pub struct XorShift {
    x: Wrapping<u32>,
    y: Wrapping<u32>,
    z: Wrapping<u32>,
    w: Wrapping<u32>,
}

impl XorShift {
    /// Create object with default seed.
    pub fn new() -> XorShift {
        XorShift {
            x: Wrapping(123_456_789),
            y: Wrapping(362_436_069),
            z: Wrapping(521_288_629),
            w: Wrapping(88_675_123),
        }
    }

    /// Get next random u32 number.
    pub fn next_u32(&mut self) -> u32 {
        let x = self.x;
        let t = x ^ (x << 11);
        self.x = self.y;
        self.y = self.z;
        self.z = self.w;
        let v = self.w;
        self.w = v ^ (v >> 19) ^ (t ^ (t >> 8));
        self.w.0
    }
}

#[cfg(test)]
mod tests {
    use super::XorShift;
    #[test]
    fn test_basic() {
        let mut xorshift = XorShift::new();
        assert_eq!(3701687786, xorshift.next_u32());
        assert_eq!(458299110, xorshift.next_u32());
    }
}
