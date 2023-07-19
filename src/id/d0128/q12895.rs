// BOJ 12895 [Fancy Village]
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

fn cnt(x: i32) -> i32 {
    let mut x = x;
    let mut r = 0;
    while x > 0 {
        r += x & 1;
        x >>= 1;
    }
    r
}
struct SegTree {
    n: usize,
    v: Vec<i32>,
    lazy: Vec<i32>,
}
impl SegTree {
    fn new(n: usize) -> Self {
        let mut m = 1;
        while m < n { m <<= 1; }
        Self { n: m, v: vec![1; m<<1], lazy: vec![0; m<<1] }
    }

    fn propagate(&mut self, x: usize, s: usize, e: usize) {
        if self.lazy[x] == 0 { return; }
        self.v[x] = self.lazy[x];
        if s < e {
            self.lazy[x<<1] = self.lazy[x];
            self.lazy[x<<1|1] = self.lazy[x];
        }
        self.lazy[x] = 0;
    }

    fn update(&mut self, x: usize, s: usize, e: usize, l: usize, r: usize, v: usize) {
        self.propagate(x, s, e);
        if r < s || e < l { return; }
        if l <= s && e <= r {
            self.lazy[x] = 1 << v;
            self.propagate(x, s, e);
        } else {
            let m = (s + e) >> 1;
            self.update(x<<1, s, m, l, r, v);
            self.update(x<<1|1, m+1, e, l, r, v);
            self.v[x] = self.v[x<<1] | self.v[x<<1|1];
        }
    }

    fn query(&mut self, x: usize, s: usize, e: usize, l: usize, r: usize) -> i32 {
        self.propagate(x, s, e);
        if r < s || e < l { return 0; }
        if l <= s && e <= r {
            self.v[x]
        } else {
            let m = (s + e) >> 1;
            self.query(x<<1, s, m, l, r) | self.query(x<<1|1, m+1, e, l, r)
        }
    }
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let (n, _, q) = (next::<usize>(&mut it), next::<usize>(&mut it), next::<usize>(&mut it));
    let mut seg = SegTree::new(n+1);
    for _ in 0..q {
        match next::<char>(&mut it) {
            'C' => {
                let (l, r, v) = (next::<usize>(&mut it), next::<usize>(&mut it), next::<usize>(&mut it));
                if l < r { seg.update(1, 1, n, l, r, v-1); }
                else { seg.update(1, 1, n, r, l, v-1); }
            },
            _ => {
                let (l, r) = (next::<usize>(&mut it), next::<usize>(&mut it));
                writeln!(so, "{}", if l < r {
                    cnt(seg.query(1, 1, n, l, r))
                } else {
                    cnt(seg.query(1, 1, n, r, l))
                }).unwrap();
            }
        }
    }
}
