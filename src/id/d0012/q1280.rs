// BOJ 1280 [Planting Trees]
// Supported by GitHub Copilot

use std::io;
use std::io::prelude::*;

fn read<T>(si: &mut T) -> String where T: Read {
    let mut s = String::new();
    si.read_to_string(&mut s).unwrap();
    s
}

fn next<T>(it: &mut std::str::SplitAsciiWhitespace) -> T where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug {
    it.next().unwrap().parse().unwrap()
}

// Sum segment tree
struct SegTree<T> {
    n: usize,
    v: Vec<T>,
    i: Vec<u32>,
}
impl<T> SegTree<T> where
    T: std::ops::AddAssign + std::ops::Add<Output=T> + Default + Copy {
    fn new(n: usize) -> Self {
        let mut m = 1;
        while m < n { m <<= 1; }
        Self { n: m, v: vec![T::default(); m<<1], i: vec![0; m<<1] }
    }
    fn update(&mut self, mut i: usize, x: T) {
        i += self.n;
        self.v[i] = x;
        self.i[i] = 1;
        while i > 1 {
            i >>= 1;
            self.v[i] = self.v[i<<1] + self.v[(i<<1)+1];
            self.i[i] = self.i[i<<1] + self.i[(i<<1)+1];
        }
    }
    fn query(&mut self, mut l: usize, mut r: usize) -> (T, i64) {
        l += self.n; r += self.n;
        let (mut rv, mut ri) = (T::default(), 0);
        while l <= r {
            if l & 1 == 1 { rv += self.v[l]; ri += self.i[l]; l += 1; }
            if r & 1 == 0 { rv += self.v[r]; ri += self.i[r]; r -= 1; }
            l >>= 1; r >>= 1;
        }
        (rv, ri as i64)
    }
}

const M: i64 = 1_000_000_007;
pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let n = next::<usize>(&mut it);
    let v = (0..n).map(|_| next::<i64>(&mut it)).collect::<Vec<_>>();

    let mut iv = v.iter().enumerate().map(|(i, &x)| (x, i)).collect::<Vec<_>>();
    iv.sort_unstable();
    let mut map = vec![0; n];
    (0..n).for_each(|i| map[iv[i].1] = i);

    let mut seg = SegTree::new(n+1);
    seg.update(map[0]+1, v[0]);

    let mut ans = 1;
    for (i, &x) in v.iter().enumerate().skip(1) {
        let loc = map[i]+1;
        let (lv, lc) = seg.query(1, loc-1);
        let (rv, rc) = seg.query(loc+1, n);

        ans *= ((lc - rc) * x + rv - lv) % M;
        ans %= M;

        seg.update(loc, x);
    }
    writeln!(so, "{}", ans).ok();
}
