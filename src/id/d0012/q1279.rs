// BOJ 1279 [Dice of My Own]
// Supported by GitHub Copilot

use std::io;
use std::io::prelude::*;
use std::collections::VecDeque;

const MOD: usize = 1_000_000_007;
pub fn main() {
    let n = io::stdin().lock().lines().next().unwrap().unwrap().parse::<usize>().unwrap() * 6;

    let mut ans = 0;
    let mut dp = vec![vec![0; 7]; 7];
    dp[0][0] = 1;
    let mut cache = VecDeque::new();
    cache.push_back(0);
    for i in 1..=n {
        if i == 7 { dp[0][0] = 0; }
        for j in 1..7 {
            if i < j { continue; }
            dp[i%7][j] = (dp[(i-1)%7][j-1] + dp[(i-j)%7][j]) % MOD;
        }
        cache.push_back(dp[i%7][6]);
        if i >= 15 {
            ans = (ans + cache.pop_front().unwrap()) % MOD;
        }
    }

    println!("{}", ans * 30 % MOD);
}
