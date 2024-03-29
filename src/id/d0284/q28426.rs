// BOJ 28426 [Additions and Divisions]
// Supported by GitHub Copilot

use std::io::{self, BufRead, Write};

fn mul(a: u64, b: u64, m: u64) -> u64 {
    (a as u128 * b as u128 % m as u128) as u64
}

fn pow(mut a: u64, mut b: u64, m: u64) -> u64 {
    let mut ret = 1;
    while b > 0 {
        if b & 1 > 0 { ret = mul(ret, a, m); }
        a = mul(a, a, m);
        b >>= 1;
    }
    ret
}

fn miller_rabin(n: u64, a: u64) -> bool {
    if n == a { return true }
    let mut d = n - 1;
    while d & 1 == 0 {
        if pow(a, d, n) == n - 1 { return true }
        d >>= 1;
    }
    let tmp = pow(a, d, n);
    return tmp == n - 1 || tmp == 1;
}

const TP: [u64; 12] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37];
fn is_prime(n: u64) -> bool {
    if n < 2 { return false }
    for &p in &TP {
        if n == p { return true }
        if !miller_rabin(n, p) { return false }
    }
    true
}

pub fn main() {
    let mut so = io::BufWriter::new(io::stdout().lock());
    let n = io::stdin().lock().lines().next().unwrap().unwrap().parse::<u64>().unwrap();
    if n == 1 { println!("2"); return; }

    let mut sum = 0;
    for i in 2..=n {
        write!(so, "{i} ").ok();
        sum += i;
    }
    let mut k = n+1;
    loop {
        if (sum + k) % 2 == 0 && is_prime((sum + k) >> 1) && (sum + k) % k > 0 {
            writeln!(so, "{}", k).ok();
            return;
        }
        k += 1;
    }
}
