// BOJ 16565 [N-Poker]
// Supported by GitHub Copilot

use std::io::{self, BufRead};

const MOD: i32 = 10007;
fn nck(n: i32, k: i32, f: &[i32], r: &[i32]) -> i32 {
    if n < k { return 0; }
    f[n as usize] * r[k as usize] % MOD * r[(n - k) as usize] % MOD
}
pub fn main() {
    let mut dp = vec![MOD; MOD as usize + 1];
    dp[1] = 1;
    (2..=MOD).for_each(|i| { dp[i as usize] = -(MOD / i) * dp[(MOD % i) as usize] % MOD; });

    let mut f = vec![1; MOD as usize + 1];
    let mut r = vec![1; MOD as usize + 1];
    (1..=MOD).for_each(|i| {
        f[i as usize] = f[i as usize - 1] * i % MOD;
        r[i as usize] = r[i as usize - 1] * dp[i as usize] % MOD;
    });

    let n = io::stdin().lock().lines().next().unwrap().unwrap().parse::<i32>().unwrap();
    let mut flag = true;
    let mut ans = 0;
    for i in 1..=n/4 {
        let c = nck(13, i, &f, &r) * nck(52 - 4 * i, n - 4 * i, &f, &r) % MOD;
        ans = (MOD + ans + if flag { c } else { -c + MOD }) % MOD;
        flag = !flag;
    }
    println!("{}", ans);
}
