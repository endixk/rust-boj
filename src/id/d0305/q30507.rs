// BOJ 30507 [Untitled]
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

const MOD: u64 = 1_000_000_003;
#[derive(Clone)]
struct Matrix {
    n: usize,
    m: Vec<Vec<u64>>,
}
impl Matrix {
    fn new(n: usize, m: Vec<Vec<u64>>) -> Self {
        Self { n, m }
    }
    fn id(n: usize) -> Self {
        let mut m = vec![vec![0; n]; n];
        for i in 0..n {
            m[i][i] = 1;
        }
        Self { n, m }
    }
    fn mul(&mut self, rhs: &Self) {
        let mut m = vec![vec![0; self.n]; self.n];
        for i in 0..self.n {
            for j in 0..self.n {
                for k in 0..self.n {
                    m[i][j] = (m[i][j] + self.m[i][k] * rhs.m[k][j]) % MOD;
                }
            }
        }
        self.m = m;
    }
    fn pow(&self, mut b: u64) -> Self {
        let mut m = Self::id(self.n);
        let mut a = self.clone();
        while b > 0 {
            if b & 1 == 1 {
                m.mul(&a);
            }
            let mut x = a.clone();
            x.mul(&a);
            a = x;
            b >>= 1;
        }
        m
    }
}

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let n = next::<u64>(&mut it);
    if n == 1 { writeln!(so, "4")?; return Ok(()); }

    let mut mat = Matrix::new(4, vec![
        vec![1, 0, 1, 1],
        vec![0, 1, 1, 1],
        vec![1, 1, 1, 0],
        vec![1, 1, 0, 1],
    ]);
    mat = mat.pow(n-2);

    writeln!(so, "{}", 4 * (
        mat.m[0][0] + mat.m[0][2] + mat.m[0][3] +
        mat.m[2][0] + mat.m[2][2] + mat.m[2][3] +
        mat.m[3][0] + mat.m[3][2] + mat.m[3][3]
        ) % MOD)?;

    Ok(())
}
