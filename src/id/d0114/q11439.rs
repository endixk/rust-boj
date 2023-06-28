// BOJ 11439 [Binomial Coefficient 5]
// Supported by GitHub Copilot

use std::io::{self, BufRead};

fn sieve(n: i64) -> Vec<i64> {
    let mut s = vec![true; n as usize + 1];
    s[0] = false; s[1] = false;
    for i in 2..=n {
        if i*i > n { break; }
        if !s[i as usize] { continue; }
        for j in (i*i..=n).step_by(i as usize) {
            s[j as usize] = false;
        }
    }
    s.iter().enumerate().filter(|(_, &b)| b).map(|(i, _)| i as i64).collect()
}

fn factorize(x: i64, p: &[i64]) -> Vec<i64> {
    let mut ret = vec![];
    for &k in p {
        let (mut f, mut c) = (k, 0);
        while f <= x {
            c += x / f;
            f *= k;
        }
        ret.push(c);
    }
    ret
}

fn pmod(a: i64, x: i64, m: i64) -> i64 {
    let mut ret = 1;
    let mut a = a % m;
    let mut x = x;
    while x > 0 {
        if x & 1 == 1 { ret = ret * a % m; }
        a = a * a % m;
        x >>= 1;
    }
    ret
}

pub fn main() {
    let s = io::stdin().lock().lines().next().unwrap().unwrap();
    let mut it = s.split_ascii_whitespace();
    let (n, k, m): (i64, i64, i64) = (
        it.next().unwrap().parse().unwrap(),
        it.next().unwrap().parse().unwrap(),
        it.next().unwrap().parse().unwrap(),
    );

    let p = sieve(n);
    let mut v = factorize(n, &p);
    factorize(k, &p).iter().enumerate().for_each(|(i, &x)| v[i] -= x);
    factorize(n-k, &p).iter().enumerate().for_each(|(i, &x)| v[i] -= x);

    println!("{}", v.iter().enumerate()
        .filter(|(_, &x)| x > 0)
        .fold(1, |acc, (i, &x)| acc * pmod(p[i], x, m) % m));
}
