// BOJ 17401 [Cells at Work]
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

#[derive(Clone)]
struct Mat {
    n: usize,
    v: Vec<Vec<i64>>,
}

fn mat_mul(a: &Mat, b: &Mat, m: i64) -> Mat {
    let mut v = vec![vec![0; a.n]; a.n];
    for i in 0..a.n {
        for j in 0..a.n {
            for k in 0..a.n {
                v[i][j] = (v[i][j] + a.v[i][k] * b.v[k][j]) % m;
            }
        }
    }
    Mat{n: a.n, v}
}

fn pow(a: &Mat, p: i64, m: i64) -> Mat {
    if p == 1 {
        return a.clone();
    }
    let b = pow(a, p / 2, m);
    let c = mat_mul(&b, &b, m);
    if p % 2 == 0 {
        c
    } else {
        mat_mul(&c, a, m)
    }
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let (t, n, d) = (
        next::<usize>(&mut it),
        next::<usize>(&mut it),
        next::<usize>(&mut it));
    let mats = (0..t).map(|_| {
        let mut v = vec![vec![0; n]; n];
        for _ in 0..next(&mut it) {
            let (a, b, c) = (
                next::<usize>(&mut it) - 1,
                next::<usize>(&mut it) - 1,
                next::<i64>(&mut it));
            v[a][b] = c;
        }
        Mat{n, v}
    }).collect::<Vec<_>>();

    if d == 0 {
        for _ in 0..n {
            for _ in 0..n {
                write!(so, "0 ").unwrap();
            }
            writeln!(so).unwrap();
        }
        return;
    }

    const MOD: i64 = 1_000_000_007;
    let mut mfac = vec![];
    mfac.push(mats[0].clone());
    for i in 1..t {
        mfac.push(mat_mul(&mfac[i - 1], &mats[i], MOD));
    }

    if d > t {
        let mut ans = pow(&mfac[t - 1], d as i64 / t as i64, MOD);
        if d % t != 0 {
            ans = mat_mul(&ans, &mfac[d % t - 1], MOD);
        }
        for i in 0..n {
            for j in 0..n {
                write!(so, "{} ", ans.v[i][j]).unwrap();
            }
            writeln!(so).unwrap();
        }
    } else {
        let ans = pow(&mfac[d - 1], 1, MOD);
        for i in 0..n {
            for j in 0..n {
                write!(so, "{} ", ans.v[i][j]).unwrap();
            }
            writeln!(so).unwrap();
        }
    }
}
