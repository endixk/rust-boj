// BOJ 1978 [Finding Primes]
// Supported by GitHub Copilot

use std::io::{self, BufRead, Write};

fn sieve(n: usize) -> Vec<bool> {
    let mut is_prime = vec![true; n+1];
    is_prime[1] = false;
    for p in 2..=n {
        if is_prime[p] {
            for i in (p*p..=n).step_by(p) {
                is_prime[i] = false;
            }
        }
    }
    is_prime
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());

    let is_prime = sieve(1000);

    si.read_line(&mut String::new()).unwrap();
    let mut buf = String::new();
    si.read_line(&mut buf).unwrap();
    writeln!(so, "{}", buf
        .split_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .filter(|&n| is_prime[n])
        .count())
        .unwrap();
}
