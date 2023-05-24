// BOJ 10830 [Matrix Exponentiation]
// Supported by GitHub Copilot

use std::io;
use std::io::prelude::*;

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

#[derive(Clone)]
struct Matrix {
    n: usize,
    m: Vec<Vec<u32>>,
}
impl Matrix {
    fn new(n: usize, m: Vec<Vec<u32>>) -> Self {
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
                    m[i][j] += self.m[i][k] * rhs.m[k][j];
                }
                m[i][j] %= 1000;
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
pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let (n, b) = (next::<usize>(&mut it), next::<u64>(&mut it));
    let m = (0..n).map(|_| {
        (0..n).map(|_| next::<u32>(&mut it)).collect::<Vec<_>>()
    }).collect::<Vec<_>>();
    let mut a = Matrix::new(n, m);
    a = a.pow(b);
    for i in 0..n {
        for j in 0..n {
            write!(so, "{} ", a.m[i][j]).unwrap();
        }
        writeln!(so).unwrap();
    }
}
