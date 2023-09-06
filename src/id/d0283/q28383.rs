// BOJ 28383 [Sum of Five Squares]
// Supported by GitHub Copilot

fn sieve(n: u64) -> Vec<u64> {
    let mut ret = Vec::new();
    let mut p = vec![true; n as usize + 1];
    p[0] = false; p[1] = false;
    for i in 2..=n {
        if p[i as usize] {
            ret.push(i);
            let mut j = i * i;
            while j <= n {
                p[j as usize] = false;
                j += i;
            }
        }
    }
    ret
}

fn factorize(n: u64, p: &[u64]) -> Vec<u64> {
    let mut ret = Vec::new();
    let mut n = n;
    for &p in p {
        if p * p > n { break }
        while n % p == 0 {
            ret.push(p);
            n /= p;
        }
    }
    if n > 1 { ret.push(n); }
    ret
}

fn divsum(fc: &[(u64, u64)]) -> u64 {
    let mut ret = 1;
    for &(f, c) in fc {
        let mut sum = 0;
        let mut p = 1;
        for _ in 0..=c {
            sum += p;
            p *= f;
        }
        ret *= sum;
    }
    ret
}

fn jacobi(n: u64, p: &[u64]) -> u64 {
    if n == 0 { return 1 }
    let mut factors = factorize(n, p);
    factors.sort_unstable_by(|a, b| b.cmp(a));

    let mut fc = Vec::<(u64, u64)>::new();
    for f in factors {
        if fc.is_empty() || fc.last().unwrap().0 != f {
            fc.push((f, 1));
        } else {
            fc.last_mut().unwrap().1 += 1;
        }
    }

    let mut ret = 8 * divsum(&fc);
    if n % 4 == 0 {
        let c = fc.len() - 1;
        fc[c].1 -= 2;
        if fc[c].1 == 0 { fc.pop(); }
        ret -= 32 * divsum(&fc);
    }
    ret
}

fn five_squares(n: u64) -> (u64, u64) {
    let p = sieve(100000);
    let four = jacobi(n, &p);
    let mut five = four;
    let mut i = 1;
    while i*i <= n {
        five += 2 * jacobi(n - i*i, &p);
        i += 1;
    }
    (four, five)
}

use std::io::{self, BufRead};
pub fn main() {
    let n = io::stdin().lock().lines().next().unwrap().unwrap().parse::<u64>().unwrap();
    let (four, five) = five_squares(n);
    println!("{} {}", four, five);
}
