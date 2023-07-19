// BOJ 17353 [Stars Falling From the Sky]
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

// Sum Segment Tree with Lazy Propagation
struct SegTree {
    n: usize,
    v: Vec<i64>,
    lazy: Vec<i64>,
}
impl SegTree {
    fn new(n: usize) -> Self {
        let mut m = 1;
        while m < n { m <<= 1; }
        Self { n: m, v: vec![0; m<<1], lazy: vec![0; m<<1] }
    }

    fn propagate(&mut self, x: usize, s: usize, e: usize) {
        if self.lazy[x] == 0 { return; }
        self.v[x] += self.lazy[x] * (e - s + 1) as i64;
        if s < e {
            self.lazy[x<<1] += self.lazy[x];
            self.lazy[x<<1|1] += self.lazy[x];
        }
        self.lazy[x] = 0;
    }

    fn update(&mut self, x: usize, s: usize, e: usize, l: usize, r: usize, v: i64) {
        self.propagate(x, s, e);
        if r < s || e < l { return; }
        if l <= s && e <= r {
            self.lazy[x] += v;
            self.propagate(x, s, e);
        } else {
            let m = (s + e) >> 1;
            self.update(x<<1, s, m, l, r, v);
            self.update(x<<1|1, m+1, e, l, r, v);
            self.v[x] = self.v[x<<1] + self.v[x<<1|1];
        }
    }

    fn query(&mut self, x: usize, s: usize, e: usize, l: usize, r: usize) -> i64 {
        self.propagate(x, s, e);
        if r < s || e < l { return 0; }
        if l <= s && e <= r {
            self.v[x]
        } else {
            let m = (s + e) >> 1;
            self.query(x<<1, s, m, l, r) + self.query(x<<1|1, m+1, e, l, r)
        }
    }
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let n = next::<usize>(&mut it);
    let v = (0..n).map(|_| next::<i64>(&mut it)).collect::<Vec<_>>();
    let mut seg = SegTree::new(n+2);

    let mut t = 0;
    for (i, x) in v.into_iter().enumerate() {
        seg.update(1, 1, n+1, i+1, i+1, x-t);
        t = x;
    }

    for _ in 0..next(&mut it) {
        match next::<u8>(&mut it) {
            1 => {
                let (l, r) = (next::<usize>(&mut it), next::<usize>(&mut it));
                seg.update(1, 1, n+1, l, r, 1);
                seg.update(1, 1, n+1, r+1, r+1, -((r-l+1) as i64));
            },
            _ => writeln!(so, "{}", seg.query(1, 1, n+1, 1, next::<usize>(&mut it))).unwrap(),
        }
    }
}
