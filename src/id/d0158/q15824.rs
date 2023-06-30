// BOJ 15824 [Capsaicin]
// Supported by GitHub Copilot

use std::io::{self, Read};

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

const MOD: i64 = 1_000_000_007;
fn pow2(mut x: i64) -> i64 {
    let mut ret = 1;
    let mut f = 2;
    while x > 0 {
        if x & 1 == 1 {
            ret *= f;
            ret %= MOD;
        }
        f *= f;
        f %= MOD;
        x >>= 1;
    }
    ret
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let n = next::<usize>(&mut it);
    let mut v = (0..n).map(|_| next::<i64>(&mut it)).collect::<Vec<_>>();
    v.sort_unstable();

    let mut ans = 0;
    for (i, &x) in v.iter().enumerate() {
        ans += x * pow2(i as i64) % MOD;
        ans -= x * pow2((n - i - 1) as i64) % MOD;
        ans %= MOD;
    }
    println!("{}", (ans + MOD) % MOD);
}
