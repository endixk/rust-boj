// BOJ 14428 [Sequence and Queries 16]
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

// Min segment tree
struct SegTree<T> {
    n: usize,
    v: Vec<T>,
    i: Vec<usize>,
}
impl<T> SegTree<T> where
    T: Ord + Default + Copy {
    fn new(n: usize) -> Self {
        let mut m = 1;
        while m < n { m <<= 1; }
        Self { n: m, v: vec![T::default(); m<<1], i: vec![0; m<<1] }
    }
    fn build(&mut self, a: &[T]) {
        for i in 0..a.len() {
            self.v[self.n+i] = a[i];
            self.i[self.n+i] = i+1;
        }
        for i in (1..self.n).rev() {
            if self.v[i<<1] <= self.v[(i<<1)+1] {
                self.v[i] = self.v[i<<1];
                self.i[i] = self.i[i<<1];
            } else {
                self.v[i] = self.v[(i<<1)+1];
                self.i[i] = self.i[(i<<1)+1];
            }
        }
    }
    fn update(&mut self, mut i: usize, x: T) {
        i += self.n;
        self.v[i] = x;
        while i > 1 {
            i >>= 1;
            if self.v[i<<1] <= self.v[(i<<1)+1] {
                self.v[i] = self.v[i<<1];
                self.i[i] = self.i[i<<1];
            } else {
                self.v[i] = self.v[(i<<1)+1];
                self.i[i] = self.i[(i<<1)+1];
            }
        }
    }
    fn query(&self, mut l: usize, mut r: usize) -> usize {
        l += self.n; r += self.n;
        let mut ans: Option<T> = None;
        let mut idx: usize = 0;
        while l <= r {
            if l & 1 == 1 {
                if let Some(x) = ans {
                    if x > self.v[l] || (x == self.v[l] && idx > self.i[l]) {
                        ans = Some(self.v[l]);
                        idx = self.i[l];
                    }
                }
                else {
                    ans = Some(self.v[l]);
                    idx = self.i[l];
                }
                l += 1;
            }
            if r & 1 == 0 {
                if let Some(x) = ans {
                    if x > self.v[r] || (x == self.v[r] && idx > self.i[r]) {
                        ans = Some(self.v[r]);
                        idx = self.i[r];
                    }
                }
                else {
                    ans = Some(self.v[r]);
                    idx = self.i[r];
                }
                r -= 1;
            }
            l >>= 1; r >>= 1;
        }
        idx
    }
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let n = next::<usize>(&mut it);
    let v = (0..n).map(|_| next::<usize>(&mut it)).collect::<Vec<_>>();
    let mut seg = SegTree::new(n);
    seg.build(&v);

    for _ in 0..next(&mut it) {
        match next(&mut it) {
            1 => seg.update(next::<usize>(&mut it)-1, next(&mut it)),
            _ => writeln!(so, "{}", seg.query(next::<usize>(&mut it)-1, next::<usize>(&mut it)-1)).unwrap(),
        }
    }
}
