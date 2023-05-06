// BOJ 28020 [Attention]
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

struct SegTree {
    n: usize,
    v: Vec<usize>,
}
impl SegTree {
    fn new(n: usize) -> Self {
        let mut m = 1;
        while m < n { m <<= 1; }
        Self { n: m, v: vec![0; m<<1] }
    }

    fn update(&mut self, mut x: usize, v: usize) {
        x += self.n;
        self.v[x] = v;
        while x > 1 {
            x >>= 1;
            self.v[x] = self.v[x<<1] + self.v[x<<1|1];
        }
    }

    fn query(&mut self, mut l: usize, mut r: usize) -> usize {
        l += self.n; r += self.n;
        let mut ans = 0;
        while l <= r {
            if l & 1 == 1 { ans += self.v[l]; l += 1; }
            if r & 1 == 0 { ans += self.v[r]; r -= 1; }
            l >>= 1; r >>= 1;
        }
        ans
    }
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let n: usize = next(&mut it);
    let a = (0..n).map(|_| next(&mut it)).collect::<Vec<usize>>();
    let b = (0..n).map(|_| next(&mut it)).collect::<Vec<usize>>();

    let mut ai = vec![0; n+1];
    for i in 0..n { ai[a[i]] = i; }
    let mut ans = 0;
    let mut seg = SegTree::new(n);
    for (i, &x) in b.iter().enumerate() {
        let loc = ai[x];
        let q = SegTree::query(&mut seg, 0, loc);
        ans += q * (if n-loc-1 > i-q {n-loc-1-i+q} else {0} );
        SegTree::update(&mut seg, loc, 1);
    }

    if ans == 0 {
        writeln!(so, "Attention is what I want").ok();
    } else {
        writeln!(so, "My heart has gone to paradise").ok();
        writeln!(so, "{}", ans).ok();
    }
}
