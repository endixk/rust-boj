// BOJ 16214 [N and M]
// Supported by GitHub Copilot

use std::io;
use std::io::prelude::*;

fn read(si: &mut io::BufReader<io::StdinLock>) -> String {
    let mut s = String::new();
    si.read_to_string(&mut s).unwrap();
    s
}

fn next<T>(it: &mut std::str::SplitAsciiWhitespace) -> T where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug {
    it.next().unwrap().parse().unwrap()
}

fn phi_prime(p: u64, pow: u64) -> u64 {
    let mut ret = 1;
    for _ in 0..pow {
        ret *= p;
    }
    ret - ret / p
}
fn phi(x: u64) -> u64 {
    let mut ret = 1;
    let mut p = 2;
    let mut x = x;
    while p * p <= x {
        let mut pow = 0;
        while x % p == 0 {
            pow += 1;
            x /= p;
        }
        ret *= phi_prime(p, pow);
        p += 1;
    }
    if x > 1 {
        ret *= phi_prime(x, 1);
    }
    ret
}
fn pow_mod(x: u64, p: u64, m: u64) -> u64 {
    let mut ret = 1;
    let mut x = x;
    let mut p = p;
    while p > 0 {
        if p & 1 == 1 {
            ret = ret * x % m;
        }
        x = x * x % m;
        p >>= 1;
    }
    ret
}

fn solve(n: u64, q: u64) -> u64 {
    if q == 1 { return 0; }
    let phi_q = phi(q);
    pow_mod(n, solve(n, phi_q) + phi_q, q)
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());

    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();
    for _ in 0..next(&mut it) {
        let (n, q) = (next(&mut it), next(&mut it));
        writeln!(so, "{}", solve(n, q)).unwrap();
    }
}
