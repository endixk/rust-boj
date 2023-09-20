// BOJ 2740 [Matrix Multiplication]
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

struct Matrix {
    n: usize,
    m: usize,
    v: Vec<Vec<i32>>,
}
impl Matrix {
    fn mul(&self, rhs: &Matrix) -> Matrix {
        let mut v = vec![vec![0; rhs.m]; self.n];
        for i in 0..self.n {
            for j in 0..rhs.m {
                for k in 0..self.m {
                    v[i][j] += self.v[i][k] * rhs.v[k][j];
                }
            }
        }
        Matrix { n: self.n, m: rhs.m, v }
    }
}

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let (n, m) = (next::<usize>(&mut it), next::<usize>(&mut it));
    let mut a = Matrix { n, m, v: vec![vec![0; m]; n] };
    for i in 0..n { for j in 0..m {
        a.v[i][j] = next::<i32>(&mut it);
    }}

    let (m, k) = (next::<usize>(&mut it), next::<usize>(&mut it));
    let mut b = Matrix { n: m, m: k, v: vec![vec![0; k]; m] };
    for i in 0..m { for j in 0..k {
        b.v[i][j] = next::<i32>(&mut it);
    }}

    let c = a.mul(&b);
    for i in 0..n {
        for j in 0..k {
            write!(so, "{} ", c.v[i][j])?;
        }
        writeln!(so)?;
    }

    Ok(())
}
