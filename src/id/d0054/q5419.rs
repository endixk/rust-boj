// BOJ 5419 [North-Western Winds]
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

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    for _ in 0..next(&mut it) {
        let n = next::<usize>(&mut it);
        let (mut x, mut y) = (Vec::new(), Vec::new());
        for i in 0..n {
            x.push((next::<i32>(&mut it), i));
            y.push((-next::<i32>(&mut it), i));
        }

        x.sort_unstable();
        y.sort_unstable();
        let mut c = vec![(0, 0); n];
        let (mut tx, mut ty, mut cx, mut cy) = (i32::MIN, i32::MIN, 0, 0);
        for i in 0..n {
            if tx != x[i].0 { tx = x[i].0; cx += 1; }
            c[x[i].1].0 = cx;
            if ty != y[i].0 { ty = y[i].0; cy += 1; }
            c[y[i].1].1 = cy;
        }

        c.sort_unstable();
        let mut seg = SegTree::<usize>::new(n+1);
        let mut ans = 0;
        for i in (0..n).rev() {
            ans += seg.query(c[i].1, n);
            seg.update(c[i].1, 1);
        }
        writeln!(so, "{}", ans)?;
    }

    Ok(())
}
