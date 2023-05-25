// BOJ 11385 [Thinksmall]
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

fn pow(mut a: i64, mut b: i64, m: i64) -> i64 {
    let mut r = 1;
    while b > 0 {
        if b & 1 == 1 {
            r = r * a % m;
        }
        a = a * a % m;
        b >>= 1;
    }
    r
}
fn ntt(v: &mut Vec<i64>, m: i64, w: i64, inv: bool) {
    let n = v.len();
    let mut i = 0;
    for j in 1..n - 1 {
        let mut k = n >> 1;
        while i & k > 0 {
            i ^= k;
            k >>= 1;
        }
        i ^= k;
        if i < j {
            v.swap(i, j);
        }
    }
    let mut step = 1;
    while step < n {
        let mut root = pow(w, (m - 1) / (step << 1) as i64, m);
        if inv {
            root = pow(root, m - 2, m);
        }
        for i in (0..n).step_by(step << 1) {
            let mut w = 1;
            for j in 0..step {
                let x = v[i + j];
                let y = v[i + j + step] * w % m;
                v[i + j] = (x + y) % m;
                v[i + j + step] = (x - y + m) % m;
                w = w * root % m;
            }
        }
        step <<= 1;
    }
    if inv {
        let inv = pow(n as i64, m - 2, m);
        for i in 0..n {
            v[i] = v[i] * inv % m;
        }
    }
}
fn multiply(mut a: Vec<i64>, mut b: Vec<i64>, m: i64, w: i64) -> Vec<i64> {
    let n = (a.len() + b.len()).next_power_of_two();
    a.resize(n, 0);
    b.resize(n, 0);
    ntt(&mut a, m, w, false);
    ntt(&mut b, m, w, false);
    for i in 0..n {
        a[i] = a[i] * b[i] % m;
    }
    ntt(&mut a, m, w, true);
    a
}

fn xgcd(a: i128, b: i128) -> (i128, i128, i128) {
    if b == 0 { return (a, 1, 0); }
    let (g, x, y) = xgcd(b, a % b);
    (g, y, x - a / b * y)
}
fn crt(a: i64, b: i64, m: i64, n: i64) -> i64 {
    let (a, b, m, n) = (a as i128, b as i128, m as i128, n as i128);
    let (_, x, y) = xgcd(m, n);
    let k = m * n;
    let mut ret = a * n % k * y % k + b * m % k * x % k;
    ret = (ret + k) % k;
    ret as i64
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let (n, m) = (next::<usize>(&mut it), next::<usize>(&mut it));
    let a = (0..=n).map(|_| next::<i64>(&mut it)).collect::<Vec<_>>();
    let b = (0..=m).map(|_| next::<i64>(&mut it)).collect::<Vec<_>>();
    let c = multiply(a.clone(), b.clone(), 998244353, 3);
    let d = multiply(a.clone(), b.clone(), 1004535809, 3);

    let mut r = vec![0; n+m+1];
    for (i, (&x, &y)) in c[..n+m+1].iter().zip(d.iter()).enumerate() {
        r[i] = crt(x, y, 998244353, 1004535809);
    }
    println!("{}", r.iter().skip(1).fold(r[0], |acc, &x| acc ^ x ));
}
