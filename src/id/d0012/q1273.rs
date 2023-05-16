// BOJ 1273 [KONZERVE]
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
    fn build(&mut self, a: &[usize]) {
        for i in 0..a.len() {
            self.v[self.n+i] = a[i];
        }
        for i in (1..self.n).rev() {
            self.v[i] = self.v[i<<1] + self.v[(i<<1)+1];
        }
    }
    fn update(&mut self, mut i: usize, x: usize) {
        i += self.n;
        self.v[i] = x;
        while i > 1 {
            i >>= 1;
            self.v[i] = self.v[i<<1] + self.v[(i<<1)+1];
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

const MAX: usize = 3_000_003;
fn imos(v: Vec<Vec<i32>>, n: usize) -> Vec<i32> {
    let mut imos = vec![0; MAX];
    for i in 0..n {
        imos[0] += 1;
        imos[v[i][0] as usize] += 1;
        imos[(v[i][0] + v[i][1]) as usize] += 3;
        imos[(v[i][0] + v[i][1] + v[i][2]) as usize] -= 5;
    }
    imos
}
fn prefix(imos: Vec<i32>) -> Vec<i32> {
    let mut ps = vec![0; MAX];
    let (mut x, mut dv) = (0, imos[0]);
    for i in 1..MAX {
        x += dv;
        ps[i] = x;
        dv += imos[i];
    }
    ps
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let n = next::<usize>(&mut it);
    let mut v = vec![vec![0; 3]; n];
    for j in 0..3 { for i in 0..n {
        v[i][j] = next(&mut it);
    }}
    let ps = prefix(imos(v, n));

    let mut st = SegTree::new(MAX);
    st.build(&vec![1; MAX]);
    for _ in 0..next(&mut it) {
        let q = next(&mut it);
        let (mut l, mut r) = (0, MAX);
        while l < r {
            let m = (l + r) >> 1;
            if st.query(0, m) < q { l = m + 1; }
            else { r = m; }
        }
        writeln!(so, "{}", ps[l+1] - ps[l]).ok();
        st.update(l, 0);
    }
}
