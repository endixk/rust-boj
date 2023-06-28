// BOJ 11401 [Binomial Coefficient 3]
// Supported by GitHub Copilot

use std::io::{self, BufRead};

const MOD: i64 = 1_000_000_007;
pub fn main() {
    let s = io::stdin().lock().lines().next().unwrap().unwrap();
    let mut it = s.split_ascii_whitespace();
    let (n, k): (usize, usize) = (it.next().unwrap().parse().unwrap(), it.next().unwrap().parse().unwrap());

    let mut ans = 1;
    let mut dp = vec![MOD as i32; n+1];
    dp[1] = 1;
    for i in 2..=n {
        ans = (ans * i as i64) % MOD;
        dp[i] = (-(MOD / i as i64) * dp[MOD as usize % i] as i64 % MOD) as i32;
        if i <= k { ans = ans * dp[i] as i64 % MOD; }
        if i <= n - k { ans = ans * dp[i] as i64 % MOD; }
    }
    ans = (ans + MOD) % MOD;
    println!("{}", ans);
}

