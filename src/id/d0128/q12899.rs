// BOJ 12899 [Data Structure]
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

struct SegTree {
    n: usize,
    v: Vec<i32>,
}
impl SegTree  {
    fn new(n: usize) -> Self {
        let mut m = 1;
        while m < n { m <<= 1; }
        Self { n: m, v: vec![0; m<<1] }
    }
    fn update(&mut self, mut i: usize, x: i32) {
        i += self.n;
        self.v[i] += x;
        while i > 1 {
            i >>= 1;
            self.v[i] = self.v[i<<1] + self.v[(i<<1)+1];
        }
    }
    fn query(&self, k: i32) -> usize {
        let mut i = 1;
        let mut k = k;
        while i < self.n {
            i <<= 1;
            if self.v[i] < k {
                k -= self.v[i];
                i += 1;
            }
        }
        i - self.n
    }
}

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let n = next::<usize>(&mut it);
    const MAX: usize = 2_000_002;
    let mut seg = SegTree::new(MAX);
    for _ in 0..n {
        let (t, x) = (next::<u8>(&mut it), next::<i32>(&mut it));
        match t {
            1 => seg.update(x as usize, 1),
            _ => {
                let k = seg.query(x);
                seg.update(k, -1);
                writeln!(so, "{}", k)?;
            }
        }
    }

    Ok(())
}
