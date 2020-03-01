// url: https://judge.yosupo.jp/problem/unionfind
use algorithms::data_structure::UnionFind;
use text_scanner::scan;

fn main() {
    let n: usize = scan();
    let q: usize = scan();

    let mut uf = UnionFind::new(n);

    for _ in 0..q {
        let (t, u, v): (usize, usize, usize) = scan();
        if t == 0 {
            uf.unite(u, v);
        } else {
            println!("{}", if uf.same(u, v) { 1 } else { 0 });
        }
    }
}
