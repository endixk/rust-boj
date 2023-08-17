// BOJ 15337 [Starting a Scenic Railroad Service]
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
    fn update(&mut self, mut i: usize, x: T) {
        i += self.n;
        self.v[i] += x;
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

const MAX: usize = 100001;
pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let n = next::<usize>(&mut it);
    let v = (0..n).map(|_| (next::<usize>(&mut it), next::<usize>(&mut it))).collect::<Vec<_>>();
    let mut sseg = SegTree::<i32>::new(MAX);
    let mut eseg = SegTree::<i32>::new(MAX);
    let mut imos = vec![0; MAX];
    for &(s, e) in &v {
        sseg.update(s, 1);
        eseg.update(e, 1);
        imos[s] += 1;
        imos[e] -= 1;
    }

    let mut p1 = 0;
    for &(s, e) in &v {
        let p = sseg.query(e, MAX-1) + eseg.query(1, s);
        if n as i32 - p > p1 { p1 = n as i32 - p; }
    }
    let (mut p2, mut x) = (0, 0);
    for i in 1..MAX {
        x += imos[i];
        if x > p2 { p2 = x; }
    }
    println!("{} {}", p1, p2);
}
