use super::mint::{Mint, Module};

pub struct Comb<M: Module> {
    fact: Vec<Mint<M>>,
    factinv: Vec<Mint<M>>,
}

impl<M: Module> Comb<M> {
    pub fn new(n: usize) -> Comb<M> {
        let mut fact: Vec<Mint<M>> = vec![0.into(); n + 1];
        let mut factinv: Vec<Mint<M>> = vec![0.into(); n + 1];
        fact[0] = 1.into();
        for i in 0..n {
            fact[i + 1] = fact[i] * (i + 1);
        }
        factinv[n] = fact[n].inv();
        for i in (0..n).rev() {
            factinv[i] = factinv[i + 1] * (i + 1);
        }
        Comb {
            fact: fact,
            factinv: factinv,
        }
    }

    pub fn fact(&self, n: u64) -> Mint<M> {
        if let Some(x) = self.fact.get(n as usize) {
            *x
        } else if n >= M::module() as u64 {
            Mint::from(0)
        } else {
            let mut res = 1.into();
            for a in 1..(n + 1) {
                res *= a;
            }
            res
        }
    }

    pub fn factinv(&self, n: u64) -> Mint<M> {
        if let Some(x) = self.factinv.get(n as usize) {
            *x
        } else {
            self.fact(n).inv()
        }
    }

    pub fn perm(&self, n: u64, r: u64) -> Mint<M> {
        if n >= r {
            self.fact(n) * self.factinv((n - r) as u64)
        } else {
            0.into()
        }
    }

    pub fn comb(&self, n: u64, r: u64) -> Mint<M> {
        let m = M::module() as u64;
        if n >= m {
            self.comb(n % m, r % m) * self.comb(n / m, r / m) // Lucas' theorem
        } else if n >= r {
            self.fact(n) * self.factinv(n - r) * self.factinv(r)
        } else {
            Mint::from(0)
        }
    }

    pub fn multi_comb(&self, n: u64, r: u64) -> Mint<M> {
        if r == 0 {
            Mint::from(1)
        } else {
            self.comb(n + r - 1, r)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple() {
        #[derive(Clone, Copy, Debug)]
        struct Mod;
        impl Module for Mod {
            fn module() -> u32 {
                1000000007
            }
        }
        let c = Comb::<Mod>::new(100);
        assert_eq!(Mint::from(336), c.perm(8, 3));
        assert_eq!(Mint::from(56), c.comb(8, 3));
        assert_eq!(Mint::from(10), c.multi_comb(3, 3));
    }

    #[test]
    fn test_fact() {
        #[derive(Clone, Copy, Debug)]
        struct Mod;
        impl Module for Mod {
            fn module() -> u32 {
                1000000007
            }
        }
        let c = Comb::<Mod>::new(100);
        let p = 8721234;
        let mut f = Mint::from(1);
        for i in 1..(p + 1) {
            f *= i;
        }
        assert_eq!(f, c.fact(p));
    }
}
