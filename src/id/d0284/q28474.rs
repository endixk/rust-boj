// BOJ 28474 [Fibonacci Coprime]
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

fn sieve(n: usize) -> Vec<u64> {
    let mut p = vec![true; n+1];
    p[0] = false;
    p[1] = false;
    for i in 2..=n {
        if p[i] {
            for j in (i*i..=n).step_by(i) {
                p[j] = false;
            }
        }
    }
    p.iter().enumerate().filter(|(_, &b)| b).map(|(i, _)| i as u64).collect()
}
fn phi_prime(p: u64, pow: u64) -> u64 {
    let mut ret = 1;
    for _ in 0..pow {
        ret *= p;
    }
    ret - ret / p
}
fn phi(x: u64, pv: &[u64]) -> u64 {
    let (mut r, mut x) = (1, x);
    for &p in pv {
        if p * p > x { break; }
        let mut pow = 0;
        while x % p == 0 { pow += 1; x /= p; }
        r *= phi_prime(p, pow);
    }
    return if x > 1 {
        r * phi_prime(x, 1)
    } else { r };
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let pv = sieve(31623);
    for _ in 0..next(&mut it) {
        let n = next::<u64>(&mut it);
        if n == 1 { writeln!(so, "0").ok(); continue; }
        if n == 2 { writeln!(so, "1").ok(); continue; }
        writeln!(so, "{}", phi(n, &pv) + if n & 1 == 0 { phi(n>>1, &pv) } else { 0 }).ok();
    }
}
