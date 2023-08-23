// BOJ 28475 [Polarizing Plates]
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

struct SegTree {
    n: usize,
    v: Vec<bool>,
}
impl SegTree {
    fn new(n: usize) -> Self {
        let mut m = 1;
        while m < n { m <<= 1; }
        Self { n: m, v: vec![false; m<<1] }
    }
    fn update(&mut self, mut i: usize, x: bool) {
        i += self.n;
        self.v[i] = x;
        while i > 1 {
            i >>= 1;
            self.v[i] = self.v[i<<1] | self.v[(i<<1)+1];
        }
    }
    fn query(&mut self, mut l: usize, mut r: usize) -> bool {
        l += self.n; r += self.n;
        let mut ans = false;
        while l <= r {
            if l & 1 == 1 { ans |= self.v[l]; l += 1; }
            if r & 1 == 0 { ans |= self.v[r]; r -= 1; }
            l >>= 1; r >>= 1;
        }
        ans
    }
}

#[inline]
fn ortho(i: u8, j: u8) -> bool {
    (i + 2) % 8 == j || (i + 6) % 8 == j
}
pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let (n, m) = (next::<usize>(&mut it), next::<usize>(&mut it));
    let mut a = (0..n).map(|_| next::<u8>(&mut it)).collect::<Vec<_>>();
    let mut seg = SegTree::new(n);
    for i in 1..n {
        seg.update(i, ortho(a[i-1], a[i]));
    }

    for _ in 0..m {
        let q = next::<u8>(&mut it);
        if q == 1 {
            let (x, y) = (next::<usize>(&mut it)-1, next::<u8>(&mut it));
            a[x] = y;
            if x > 0 { seg.update(x, ortho(a[x-1], a[x])); }
            if x < n-1 { seg.update(x+1, ortho(a[x], a[x+1])); }
        } else {
            let (x, y) = (next::<usize>(&mut it)-1, next::<usize>(&mut it)-1);
            writeln!(so, "{}", if seg.query(x+1, y) { "0" } else { "1" }).unwrap();
        }
    }
}
