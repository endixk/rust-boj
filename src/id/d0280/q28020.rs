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

struct FenwickTree {
    n: usize,
    ft: Vec<i64>,
}
impl FenwickTree {
    fn new(n: usize) -> Self {
        Self {
            n,
            ft: vec![0; n+1],
        }
    }
    fn sum(&self, mut i: usize) -> i64 {
        let mut sum = 0;
        while i > 0 {
            sum += self.ft[i];
            i -= i & (!i + 1);
        }
        sum
    }
    fn add(&mut self, mut i: usize, v: i64) {
        while i <= self.n {
            self.ft[i] += v;
            i += i & (!i + 1);
        }
    }

    fn point_update(&mut self, i: usize, x: i64) {
        self.add(i, x);
    }
    fn range_query(&self, l: usize, r: usize) -> i64 {
        self.sum(r) - self.sum(l-1)
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
    let mut ft = FenwickTree::new(n+1);
    for (i, &x) in b.iter().enumerate() {
        let loc = ai[x];
        let q = ft.range_query(1, loc+1) as usize;
        ans += q * (if n-loc-1 > i-q {n-loc-1-i+q} else {0} );
        ft.point_update(loc+1, 1);
    }

    if ans == 0 {
        writeln!(so, "Attention is what I want").ok();
    } else {
        writeln!(so, "My heart has gone to paradise").ok();
        writeln!(so, "{}", ans).ok();
    }
}
