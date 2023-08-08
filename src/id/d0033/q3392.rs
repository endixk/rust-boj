// BOJ 3392 [Mars Maps]
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

// Count segment tree
struct CountSegTree {
    n: usize,
    c: Vec<i32>,
    v: Vec<usize>,
}
impl CountSegTree {
    fn new(n: usize) -> Self {
        let mut m = 1;
        while m < n { m <<= 1; }
        Self { n: m, c: vec![0; m<<1], v: vec![0; m<<1] }
    }
    fn update(&mut self, i: usize, s: usize, e: usize, l: usize, r: usize, x: i32) {
        if r < s || e < l { return; }
        if l <= s && e <= r {
            self.c[i] += x;
        } else {
            let m = (s + e) >> 1;
            self.update(i<<1, s, m, l, r, x);
            self.update((i<<1)+1, m+1, e, l, r, x);
        }
        if self.c[i] > 0 {
            self.v[i] = e - s + 1;
        } else {
            if s == e { self.v[i] = 0; }
            else { self.v[i] = self.v[i<<1] + self.v[(i<<1)+1]; }
        }
    }
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let n = next::<usize>(&mut it);
    let mut vers = Vec::new();
    for _ in 0..n {
        let (x1, y1, x2, y2) = (
            next::<usize>(&mut it), next::<usize>(&mut it),
            next::<usize>(&mut it), next::<usize>(&mut it));
        vers.push((x1, y1+1, y2, 1));
        vers.push((x2, y1+1, y2, -1));
    }
    vers.sort_unstable();

    let (mut xcur, mut ans) = (0, 0);
    let mut seg = CountSegTree::new(30003);
    for (x, y1, y2, add) in vers {
        ans += (x - xcur) * seg.v[1];
        seg.update(1, 1, 30001, y1, y2, add);
        xcur = x;
    }
    println!("{}", ans);
}
