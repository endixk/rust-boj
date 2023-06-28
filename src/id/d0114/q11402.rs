// BOJ 11402 [Binomial Coefficient 4]
// Supported by GitHub Copilot

use std::io::{self, BufRead};

fn nck(n: i64, k: i64, m: i64, f: &[i64], r: &[i64]) -> i64 {
    if n < k { return 0; }
    f[n as usize] * r[k as usize] % m * r[(n - k) as usize] % m
}
pub fn main() {
    let s = io::stdin().lock().lines().next().unwrap().unwrap();
    let mut it = s.split_ascii_whitespace();
    let (mut n, mut k, m): (i64, i64, i64) = (
        it.next().unwrap().parse().unwrap(),
        it.next().unwrap().parse().unwrap(),
        it.next().unwrap().parse().unwrap(),
    );

    let mut dp = vec![m; m as usize + 1];
    dp[1] = 1;
    (2..=m).for_each(|i| { dp[i as usize] = -(m / i) * dp[(m % i) as usize] % m; });

    let mut f = vec![1; m as usize + 1];
    let mut r = vec![1; m as usize + 1];
    (1..=m).for_each(|i| {
        f[i as usize] = f[i as usize - 1] * i % m;
        r[i as usize] = r[i as usize - 1] * dp[i as usize] % m;
    });

    let mut ans = 1;
    while n > 0 || k > 0 {
        let (a, b) = (n % m, k % m);
        ans = ans * nck(a, b, m, &f, &r) % m;
        n /= m;
        k /= m;
    }
    println!("{}", (ans + m) % m);
}
