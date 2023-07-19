// BOJ 17409 [Number of Increasing Subsequences]
// Supported by GitHub Copilot

use std::io::{self, Read};
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

const MOD: i64 = 1_000_000_007;
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
            sum = (sum + self.ft[i]) % MOD;
            i -= i & (!i + 1);
        }
        sum
    }
    fn add(&mut self, mut i: usize, v: i64) {
        while i <= self.n {
            self.ft[i] = (self.ft[i] + v) % MOD;
            i += i & (!i + 1);
        }
    }

    fn point_update(&mut self, i: usize, x: i64) {
        self.add(i, x);
    }
    fn range_query(&self, l: usize, r: usize) -> i64 {
        (self.sum(r) - self.sum(l-1) + MOD) % MOD
    }
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let (n, k) = (next::<usize>(&mut it), next::<usize>(&mut it));
    let v = (0..n).map(|_| next::<usize>(&mut it)).collect::<Vec<_>>();
    let mut a = vec![1; n];
    for _ in 1..k {
        let mut ft = FenwickTree::new(n+1);
        let mut t = vec![0; n];
        for i in 0..n {
            t[i] = ft.range_query(1, v[i]-1);
            ft.point_update(v[i], a[i]);
        }
        a = t;
    }
    println!("{}", a.iter().sum::<i64>() % MOD);
}

