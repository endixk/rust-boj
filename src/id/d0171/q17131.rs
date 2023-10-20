// BOJ 17131 [Fox]
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

    let n = next::<usize>(&mut it);
    let mut v = vec![vec![]; 400004];
    for _ in 0..n {
        let (x, y) = (next::<i32>(&mut it), next::<i32>(&mut it));
        let (x, y) = ((x + 200001) as usize, (y + 200001) as usize);
        v[y].push(x);
    }

    let mut seg = SegTree::<usize>::new(400004);
    let mut ans = 0;
    const MOD: usize = 1_000_000_007;
    for y in (0..400004).rev() {
        for &x in &v[y] {
            ans = (ans + seg.query(0, x-1) * seg.query(x+1, 400003)) % MOD;
        }
        for &x in &v[y] {
            seg.update(x, 1);
        }
    }
    writeln!(so, "{}", ans)?;

    Ok(())
}
