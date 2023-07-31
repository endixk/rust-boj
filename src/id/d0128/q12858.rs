// BOJ 12858 [Range GCD]
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

    fn range_update(&mut self, l: usize, r: usize, v: i64) {
        self.add(l, v);
        self.add(r+1, -v);
    }
    fn point_query(&self, i: usize) -> i64 {
        self.sum(i)
    }
}

// GCD segment tree
fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 { a } else { gcd(b, a % b) }
}
struct SegTree {
    n: usize,
    v: Vec<i64>,
}
impl SegTree {
    fn new(n: usize) -> Self {
        let mut m = 1;
        while m < n { m <<= 1; }
        Self { n: m, v: vec![0; m<<1] }
    }
    fn build(&mut self, a: &[i64]) {
        for i in 0..a.len() {
            self.v[self.n+i] = a[i];
        }
        for i in (1..self.n).rev() {
            self.v[i] = gcd(self.v[i<<1].abs(), self.v[(i<<1)+1].abs());
        }
    }
    fn update(&mut self, mut i: usize, x: i64) {
        i += self.n;
        self.v[i] += x;
        while i > 1 {
            i >>= 1;
            self.v[i] = gcd(self.v[i<<1].abs(), self.v[(i<<1)+1].abs());
        }
    }
    fn query(&mut self, mut l: usize, mut r: usize) -> i64 {
        l += self.n; r += self.n;
        let mut ans = 0;
        while l <= r {
            if l & 1 == 1 { ans = gcd(self.v[l].abs(), ans); l += 1; }
            if r & 1 == 0 { ans = gcd(self.v[r].abs(), ans); r -= 1; }
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

    let n = next::<usize>(&mut it);
    let v = (0..n).map(|_| next::<i64>(&mut it)).collect::<Vec<_>>();

    let mut ft = FenwickTree::new(n);
    (0..n).for_each(|i| ft.range_update(i+1, i+1, v[i]));

    let mut seg = SegTree::new(n-1);
    seg.build(&(1..n).map(|i| v[i] - v[i-1]).collect::<Vec<_>>());

    for _ in 0..next(&mut it) {
        match next::<i64>(&mut it) {
            0 => {
                let (l, r) = (next::<usize>(&mut it), next::<usize>(&mut it));
                writeln!(so, "{}", gcd(ft.point_query(l), seg.query(l-1, r-2))).ok();
            },
            x => {
                let (l, r) = (next::<usize>(&mut it), next::<usize>(&mut it));
                ft.range_update(l, r, x);
                if l > 1 { seg.update(l-2, x); }
                if r < n { seg.update(r-1, -x); }
            }
        }
    }
}
