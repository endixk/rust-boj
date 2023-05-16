// BOJ 1275 [Coffee Shop 2]
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
}
impl<T> SegTree<T> where
    T: std::ops::AddAssign + std::ops::Add<Output=T> + Default + Copy {
    fn new(n: usize) -> Self {
        let mut m = 1;
        while m < n { m <<= 1; }
        Self { n: m, v: vec![T::default(); m<<1] }
    }
    fn build(&mut self, a: &[T]) {
        for i in 0..a.len() {
            self.v[self.n+i] = a[i];
        }
        for i in (1..self.n).rev() {
            self.v[i] = self.v[i<<1] + self.v[(i<<1)+1];
        }
    }
    fn update(&mut self, mut i: usize, x: T) {
        i += self.n;
        self.v[i] = x;
        while i > 1 {
            i >>= 1;
            self.v[i] = self.v[i<<1] + self.v[(i<<1)+1];
        }
    }
    fn query(&mut self, mut l: usize, mut r: usize) -> T {
        l += self.n; r += self.n;
        let mut ans = T::default();
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

    let (n, q) = (next::<usize>(&mut it), next::<usize>(&mut it));
    let v = (0..n).map(|_| next::<i64>(&mut it)).collect::<Vec<_>>();

    let mut st = SegTree::new(n);
    st.build(&v);
    for _ in 0..q {
        let (x, y, a, b) = (
            next::<usize>(&mut it), next::<usize>(&mut it),
            next::<usize>(&mut it), next::<i64>(&mut it));
        writeln!(so, "{}", if x > y { st.query(y-1, x-1) } else { st.query(x-1, y-1) }).ok();
        st.update(a-1, b);
    }
}
