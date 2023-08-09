// BOJ 28138 [Remainder]
// Supported by GitHub Copilot

use std::io::{self, BufRead};

fn sieve(x: u64) -> Vec<u64> {
    let mut v = vec![true; x as usize + 1];
    v[0] = false;
    v[1] = false;
    for i in 2..=x {
        if v[i as usize] {
            for j in (i * i..=x).step_by(i as usize) {
                v[j as usize] = false;
            }
        }
    }
    v.iter().enumerate().filter(|(_, &b)| b).map(|(i, _)| i as u64).collect()
}
fn factorize(x: u64, p: Vec<u64>) -> (Vec<u64>, Vec<u8>, u64) {
    let mut v = vec![0; p.len()];
    let mut x = x;
    for (i, &pi) in p.iter().enumerate() {
        while x % pi == 0 {
            v[i] += 1;
            x /= pi;
        }
    }

    let (mut f, mut fp) = (Vec::new(), Vec::new());
    for (i, &vi) in v.iter().enumerate() {
        if vi > 0 {
            f.push(p[i]);
            fp.push(vi);
        }
    }
    (f, fp, x)
}

fn go(p: &[u64], f: &[u8], i: usize, c: usize, r: u64, x: u64) -> u64 {
    if i == c {
        return if x > r { x } else { 0 };
    }
    let (mut ret, mut fac) = (0, 1);
    for _ in 0..=f[i] {
        ret += go(p, f, i+1, c, r, x * fac);
        fac *= p[i];
    }
    ret
}
pub fn main() {
    let v = io::stdin().lock().lines().next().unwrap().unwrap()
        .split_ascii_whitespace().map(|x| x.parse::<u64>().unwrap()).collect::<Vec<_>>();
    let (n, r) = (v[0], v[1]);

    let p = sieve(1_000_000);
    let (mut p, mut f, x) = factorize(n-r, p);
    if x > 1 {
        p.push(x);
        f.push(1);
    }
    println!("{}", go(&p, &f, 0, p.len(), r, 1));
}
