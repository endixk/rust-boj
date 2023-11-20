// BOJ 30512 [Forgotten Elements]
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

// Minimum Segment Tree with Lazy Propagation
struct MinSegTree {
    n: usize,
    v: Vec<usize>,
    lazy: Vec<usize>,
}
const INF: usize = 0x3f3f3f3f3f3f3f3f;
impl MinSegTree {
    fn new(n: usize) -> Self {
        let mut m = 1;
        while m < n { m <<= 1; }
        Self { n: m, v: vec![INF; m<<1], lazy: vec![INF; m<<1] }
    }

    fn propagate(&mut self, x: usize, s: usize, e: usize) {
        if self.lazy[x] == INF { return; }
        self.v[x] = self.v[x].min(self.lazy[x]);
        if s < e {
            self.lazy[x<<1] = self.lazy[x].min(self.lazy[x<<1]);
            self.lazy[x<<1|1] = self.lazy[x].min(self.lazy[x<<1|1]);
        }
        self.lazy[x] = INF;
    }

    fn update(&mut self, x: usize, s: usize, e: usize, l: usize, r: usize, v: usize) {
        self.propagate(x, s, e);
        if r < s || e < l { return; }
        if l <= s && e <= r {
            self.lazy[x] = v;
            self.propagate(x, s, e);
        } else {
            let m = (s + e) >> 1;
            self.update(x<<1, s, m, l, r, v);
            self.update(x<<1|1, m+1, e, l, r, v);
            self.v[x] = self.v[x<<1].min(self.v[x<<1|1]);
        }
    }

    fn query(&mut self, x: usize, s: usize, e: usize, l: usize, r: usize) -> usize {
        self.propagate(x, s, e);
        if r < s || e < l { return INF; }
        if l <= s && e <= r {
            self.v[x]
        } else {
            let m = (s + e) >> 1;
            self.query(x<<1, s, m, l, r).min(self.query(x<<1|1, m+1, e, l, r))
        }
    }
}

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let n = next::<usize>(&mut it);
    let mut seg = MinSegTree::new(n+1);
    for i in 1..=n {
        seg.update(1, 1, n, i, i, next::<usize>(&mut it) << 18);
    }

    let q = next::<usize>(&mut it);
    for i in 1..=q {
        let (l, r, x) = (next::<usize>(&mut it), next::<usize>(&mut it), next::<usize>(&mut it));
        seg.update(1, 1, n, l, r, x << 18 | i);
    }

    let mut d = vec![0; q+1];
    for i in 1..=n {
        let x = seg.query(1, 1, n, i, i);
        write!(so, "{} ", x >> 18)?;
        d[x & 0x3ffff] += 1;
    }
    writeln!(so)?;

    let mut x = d[0];
    for i in 1..=q {
        x += d[i];
        write!(so, "{} ", x)?;
    }
    writeln!(so)?;

    Ok(())
}
