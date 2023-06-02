// BOJ 12850 [Campus Tour 2]
// Supported by GitHub Copilot

use std::io::{self, BufRead};

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
                    m[i][j] += self.m[i][k] * rhs.m[k][j];
                    m[i][j] %= 1_000_000_007;
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

pub fn main() {
    let mat = Matrix::new(
        8,
        vec![
            vec![0, 1, 0, 1, 0, 0, 0, 0],
            vec![1, 0, 1, 1, 0, 0, 0, 0],
            vec![0, 1, 0, 1, 1, 1, 0, 0],
            vec![1, 1, 1, 0, 0, 1, 0, 0],
            vec![0, 0, 1, 0, 0, 1, 1, 0],
            vec![0, 0, 1, 1, 1, 0, 0, 1],
            vec![0, 0, 0, 0, 1, 0, 0, 1],
            vec![0, 0, 0, 0, 0, 1, 1, 0],
        ],
    );

    let mat = mat.pow(io::stdin().lock().lines().next().unwrap().unwrap().parse().unwrap());
    println!("{}", mat.m[0][0]);
}
