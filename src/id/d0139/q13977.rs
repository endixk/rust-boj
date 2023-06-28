// BOJ 13977 [Binomial Coefficient and Queries]
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

const MAX: usize = 4_000_000;
const MOD: i64 = 1_000_000_007;
pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let mut dp = vec![MOD as i32; MAX + 1];
    let mut fact = vec![0; MAX + 1];
    let mut rfct = vec![0; MAX + 1];
    dp[1] = 1; fact[0] = 1; rfct[0] = 1; fact[1] = 1; rfct[1] = 1;
    for i in 2..=MAX {
        dp[i] = (-(MOD / i as i64) * dp[MOD as usize % i] as i64 % MOD) as i32;
        fact[i] = (fact[i - 1] as i64 * i as i64 % MOD) as i32;
        rfct[i] = (rfct[i - 1] as i64 * dp[i] as i64 % MOD) as i32;
    }

    for _ in 0..next(&mut it) {
        let (n, k) = (next::<usize>(&mut it), next::<usize>(&mut it));
        let ans = fact[n] as i64 * rfct[k] as i64 % MOD * rfct[n - k] as i64 % MOD;
        writeln!(so, "{}", if ans < 0 { ans + MOD } else { ans }).unwrap();
    }
}
