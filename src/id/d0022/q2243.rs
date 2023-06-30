// BOJ 2243 [Candy Box]
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

// Sum segment tree
struct SegTree<T> {
    n: usize,
    v: Vec<T>,
}
impl<T> SegTree<T> where
    T: std::ops::AddAssign + std::ops::Add<Output=T>
    + std::ops::SubAssign + std::ops::Sub<Output=T>
    + Default + Copy + PartialOrd {
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
        self.v[i] += x;
        while i > 1 {
            i >>= 1;
            self.v[i] = self.v[i<<1] + self.v[(i<<1)+1];
        }
    }
    fn query(&mut self, mut lim: T) -> usize {
        let mut i = 1;
        while i < self.n {
            i <<= 1;
            if self.v[i] < lim {
                lim -= self.v[i];
                i += 1;
            }
        }
        i - self.n
    }
}

const SIZE: usize = 1_000_001;
pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let mut seg = SegTree::<i32>::new(SIZE);
    for _ in 0..next(&mut it) {
        let q = next::<u8>(&mut it) == 1;
        if q {
            let l = seg.query(next(&mut it));
            writeln!(so, "{}", l).ok();
            seg.update(l, -1);
        } else {
            seg.update(next(&mut it), next(&mut it));
        }
    }
}
