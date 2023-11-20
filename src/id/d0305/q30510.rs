// BOJ 30510 [Thomae's Function]
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
fn phi_prime(dp: &mut [u64], p: u64, pow: u64) -> u64 {
    let x = p.pow(pow as u32) as usize;
    if dp[x] != 0 { return dp[x]; }
    let mut ret = 1;
    for _ in 0..pow {
        ret *= p;
    }
    dp[x] = ret - ret / p;
    dp[x]
}
fn phi(dp: &mut [u64], x: u64, pv: &[u64]) -> u64 {
    let (mut r, mut x) = (1, x);
    for &p in pv {
        if p * p > x { break; }
        let mut pow = 0;
        while x % p == 0 { pow += 1; x /= p; }
        r *= phi_prime(dp, p, pow);
    }
    return if x > 1 {
        r * phi_prime(dp, x, 1)
    } else { r };
}

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let (p, q) = (next::<u64>(&mut it), next::<u64>(&mut it));
    let v = sieve(317);
    let mut ans = 2;
    let mut dp = vec![0; 100001];
    for x in 2..=q {
        if q >= x * p {
            ans += phi(&mut dp, x, &v);
        }
    }
    writeln!(so, "{}", ans)?;

    Ok(())
}
