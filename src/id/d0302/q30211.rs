// BOJ 30211 [Barracks Construction]
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

struct MaxSegTree<T> {
    n: usize,
    v: Vec<T>,
}
impl<T> MaxSegTree<T> where
    T: Ord + Default + Copy {
    fn new(n: usize) -> Self {
        let mut m = 1;
        while m < n { m <<= 1; }
        Self { n: m, v: vec![T::default(); m<<1] }
    }
    fn update(&mut self, mut i: usize, x: T) {
        i += self.n;
        self.v[i] = x;
        while i > 1 {
            i >>= 1;
            self.v[i] = self.v[i<<1].max(self.v[(i<<1)+1]);
        }
    }
    fn query(&self, mut l: usize, mut r: usize) -> T {
        l += self.n; r += self.n;
        let mut ans: Option<T> = None;
        while l <= r {
            if l & 1 == 1 {
                if let Some(x) = ans { ans = Some(x.max(self.v[l])); }
                else { ans = Some(self.v[l]); }
                l += 1;
            }
            if r & 1 == 0 {
                if let Some(x) = ans { ans = Some(x.max(self.v[r])); }
                else { ans = Some(self.v[r]); }
                r -= 1;
            }
            l >>= 1; r >>= 1;
        }
        ans.unwrap()
    }
}

#[inline] fn cost(p: &[i64], q: &[i64], l: usize, r: usize) -> i64 {
    let (ps, qs) = (p[r] - p[l-1], q[r] - q[l-1]);
    let avg = (ps as f64 / (r - l + 1) as f64).round() as i64;
    qs - 2 * avg * ps + avg * avg * (r - l + 1) as i64
}
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let (n, m) = (next::<usize>(&mut it), next::<i64>(&mut it));
    let mut h = vec![0; n+1];
    for i in 1..=n { h[i] = next::<i64>(&mut it); }
    let (mut p, mut q) = (vec![0; n+1], vec![0; n+1]);
    for i in 1..=n {
        p[i] = p[i-1] + h[i];
        q[i] = q[i-1] + h[i] * h[i];
    }

    let mut opt = vec![0; n+1];
    let mut seg = MaxSegTree::new(n+1);
    let mut r = 0;
    for i in 1..=n {
        while r < n && cost(&p, &q, i, r+1) <= m { r += 1; }
        opt[i] = r;
        seg.update(i, opt[i] - i + 1);
    }

    for _ in 0..next(&mut it) {
        let (l, r) = (next::<usize>(&mut it), next::<usize>(&mut it));
        let x = opt[l..=r].partition_point(|&y| y <= r) + l;
        let mut ans = r + 1 - x;
        if x > l { ans = ans.max(seg.query(l, x - 1)); }
        writeln!(so, "{}", ans)?;
    }

    Ok(())
}
