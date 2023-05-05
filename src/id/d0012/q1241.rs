// BOJ 1241 [Knocking Heads]
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

fn sieve(primes: &mut Vec<usize>, n: usize) {
    let mut is_prime = vec![true; n + 1];
    is_prime[0] = false;
    is_prime[1] = false;
    for i in 2..=n {
        if is_prime[i] {
            primes.push(i);
            let mut j = i * i;
            while j <= n {
                is_prime[j] = false;
                j += i;
            }
        }
    }
}

fn factorize(mut n: usize, primes: &Vec<usize>) -> Vec<(usize, usize)> {
    let mut factors = Vec::new();
    for &p in primes {
        if p * p > n {
            break;
        }
        if n % p == 0 {
            let mut e = 0;
            while n % p == 0 {
                n /= p;
                e += 1;
            }
            factors.push((p, e));
        }
    }
    if n > 1 {
        factors.push((n, 1));
    }
    factors
}

fn divisors(factors: &Vec<(usize, usize)>, i: usize, d: usize, divs: &mut Vec<usize>) {
    if i == factors.len() {
        divs.push(d);
    } else {
        let (p, e) = factors[i];
        for j in 0..=e {
            divisors(factors, i + 1, d * p.pow(j as u32), divs);
        }
    }
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());

    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();
    let n: usize = next(&mut it);
    let a = (0..n).map(|_| next(&mut it)).collect::<Vec<usize>>();

    let mut cnt = vec![0; 1000001];
    for &x in &a { cnt[x] += 1; }
    let mut primes = Vec::new();
    sieve(&mut primes, n);

    for x in a {
        let mut ans = 0;
        let factors = factorize(x, &primes);
        let mut divs = Vec::new();
        divisors(&factors, 0, 1, &mut divs);
        for d in divs {
            ans += cnt[d];
        }
        writeln!(so, "{}", ans - 1).unwrap();
    }
}
