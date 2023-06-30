// BOJ 11505 [Interval Product]
// Supported by GitHub Copilot

use std::io::{self, Read, Write};

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

const MOD: u64 = 1_000_000_007;
struct SegTree {
    n: usize,
    v: Vec<u64>,
}
impl SegTree {
    fn new(n: usize) -> Self {
        let mut m = 1;
        while m < n { m <<= 1; }
        Self { n: m, v: vec![1; m<<1] }
    }
    fn build(&mut self, a: &[u64]) {
        for i in 0..a.len() {
            self.v[self.n+i] = a[i];
        }
        for i in (1..self.n).rev() {
            self.v[i] = (self.v[i<<1] * self.v[(i<<1)+1]) % MOD;
        }
    }
    fn update(&mut self, mut i: usize, x: u64) {
        i += self.n;
        self.v[i] = x;
        while i > 1 {
            i >>= 1;
            self.v[i] = (self.v[i<<1] * self.v[(i<<1)+1]) % MOD;
        }
    }
    fn query(&mut self, mut l: usize, mut r: usize) -> u64 {
        l += self.n; r += self.n;
        let mut ans = 1;
        while l <= r {
            if l & 1 == 1 { ans = (ans * self.v[l]) % MOD; l += 1; }
            if r & 1 == 0 { ans = (ans * self.v[r]) % MOD; r -= 1; }
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

    let (n, m, k) = (next::<usize>(&mut it), next::<usize>(&mut it), next::<usize>(&mut it));
    let v = (0..n).map(|_| next::<u64>(&mut it)).collect::<Vec<_>>();
    let mut seg = SegTree::new(n);
    seg.build(&v);

    for _ in 0..m+k {
        match next::<u8>(&mut it) {
            1 => seg.update(next::<usize>(&mut it)-1, next::<u64>(&mut it)),
            _ => writeln!(so, "{}", seg.query(next::<usize>(&mut it)-1, next::<usize>(&mut it)-1)).unwrap(),
        }
    }
}
