pub struct UnionFind {
    data: Vec<i32>,
}

impl UnionFind {
    pub fn new(n: usize) -> UnionFind {
        UnionFind { data: vec![-1; n] }
    }

    pub fn unite(&mut self, x: usize, y: usize) -> bool {
        let x = self.root(x);
        let y = self.root(y);
        if x != y {
            let (x, y) = if self.data[x] <= self.data[y] {
                (x, y)
            } else {
                (y, x)
            };
            self.data[x] += self.data[y];
            self.data[y] = x as i32;
        }
        x != y
    }

    pub fn same(&mut self, x: usize, y: usize) -> bool {
        self.root(x) == self.root(y)
    }

    pub fn size(&mut self, x: usize) -> u32 {
        let r = self.root(x);
        (-self.data[r]) as u32
    }

    pub fn root(&mut self, x: usize) -> usize {
        if self.data[x] < 0 {
            x
        } else {
            let nx = self.data[x] as usize;
            let r = self.root(nx);
            self.data[x] = r as i32;
            r
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple() {
        let mut uf = UnionFind::new(3);
        assert_eq!(1, uf.size(0));
        assert_eq!(1, uf.size(1));
        assert_eq!(1, uf.size(2));
        assert_eq!(0, uf.root(0));
        assert_eq!(1, uf.root(1));
        assert_eq!(2, uf.root(2));
        assert!(!uf.same(1, 2));
        uf.unite(1, 2);
        assert_ne!(uf.root(0), uf.root(2));
        assert_eq!(uf.root(1), uf.root(2));
        assert_eq!(1, uf.size(0));
        assert_eq!(2, uf.size(1));
        assert_eq!(2, uf.size(2));
        assert!(uf.same(1, 2));
        uf.unite(0, 1);
        assert_eq!(uf.root(0), uf.root(2));
        assert_eq!(uf.root(1), uf.root(2));
        assert_eq!(3, uf.size(0));
        assert_eq!(3, uf.size(1));
        assert_eq!(3, uf.size(2));
        assert!(uf.same(1, 2));
        assert!(uf.same(0, 2));
    }
}