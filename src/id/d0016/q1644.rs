// BOJ 1644 [Sum of Consecutive Prime Numbers]
// Supported by GitHub Copilot

use std::io;
use std::io::prelude::*;
use std::collections::VecDeque;

fn sieve(n: usize) -> Vec<usize> {
    let mut v = vec![true; n + 1];
    v[0] = false;
    v[1] = false;
    for i in 2..=n {
        if v[i] {
            for j in (i * i..=n).step_by(i) {
                v[j] = false;
            }
        }
    }
    v.iter().enumerate().filter_map(|(i, &b)| if b { Some(i as usize) } else { None }).collect()
}

pub fn main() {
    let n = io::stdin().lock().lines().next().unwrap().unwrap().parse::<usize>().unwrap();
    let primes = sieve(n);

    let mut dp = vec![0; n + 1];
    let mut q = VecDeque::new();
    q.push_front(0);
    for p in primes {
        let mut nq = VecDeque::new();
        while let Some(x) = q.pop_front() {
            if x + p > n { break; }
            dp[x + p] += 1;
            nq.push_back(x + p);
        }
        nq.push_front(0);
        q = nq;
    }

    println!("{}", dp[n]);
}
