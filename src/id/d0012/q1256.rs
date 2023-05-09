// BOJ 1256 [Dictionary]
// Supported by GitHub Copilot

use std::io;
use std::io::prelude::*;

const MAX: u64 = 1_000_000_000;
fn ncr(n: u64, r: u64) -> Option<u64> {
    if r > n/2 { return ncr(n, n-r); }
    let mut ret = 1u64;
    for i in 0..r {
        ret = ret.checked_mul(n - i)?;
        ret = ret.checked_div(i + 1)?;
    }
    return if ret > MAX { None } else { Some(ret) };
}

pub fn main() {
    let s = io::stdin().lock().lines().next().unwrap().unwrap();
    let (mut n, mut m, mut k) = {
        let mut it = s.split_whitespace().map(|x| x.parse::<u64>().unwrap());
        (it.next().unwrap(), it.next().unwrap(), it.next().unwrap())
    };

    let mut memo = vec![vec![None; m as usize + 1]; (n + m) as usize + 1];
    for i in 0..=n+m {
        for j in 0..=m {
            if j > i { continue }
            memo[i as usize][j as usize] = ncr(i, j);
        }
    }
    if let Some(x) = memo[(n + m) as usize][m as usize] {
        if x < k {
            println!("-1");
            return;
        }
    }

    let mut ans = String::new();
    while n > 0 && m > 0 {
        if let Some(x) = memo[(n + m - 1) as usize][m as usize] {
            if x >= k { ans.push('a'); n -= 1; }
            else { ans.push('z'); m -= 1; k -= x; }
        } else {
            ans.push('a'); n -= 1;
        }
    }

    while n > 0 { ans.push('a'); n -= 1; }
    while m > 0 { ans.push('z'); m -= 1; }
    println!("{}", ans);
}
